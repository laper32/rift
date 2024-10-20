#include "rift/coreclr/CoreCLRDelegates.h"
#include "rift/coreclr/Hostfxr.h"

#include "rift/fundamental/PrimitivePath.h"


#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif

#ifdef _WIN32
#include <windows.h>
#define STR(s) L##s
#else
#include <dlfcn.h>
#define STR(s) s
#endif

#include <algorithm>
#include <array>
#include <cassert>
#include <charconv>
#include <filesystem>
#include <iostream>
#include <optional>
#include <ranges>
#include <string>
#include <vector>


#undef LoadLibrary

namespace rift::coreclr {

struct HostfxrUtils
{
    hostfxr_initialize_for_runtime_config_fn Init;
    hostfxr_get_runtime_delegate_fn GetDelegate;
    hostfxr_close_fn Close;
} g_HostFxrUtils;

void* LoadLibrary(const char* path)
{
#ifdef _WIN32
    HMODULE h = LoadLibraryA(path);
    assert(h != nullptr);
    return (void*)h;
#else
    void* h = dlopen(path, RTLD_LAZY | RTLD_LOCAL);
    assert(h != nullptr);
    return h;
#endif
}

void* GetExport(void* h, const char* name)
{
#ifdef _WIN32
    void* f = ::GetProcAddress((HMODULE)h, name);
    assert(f != nullptr);
    return f;
#else

    void* f = dlsym(h, name);
    assert(f != nullptr);
    return f;
#endif
}

struct Version
{
    Version() = default;
    explicit Version(std::string_view input)
    {
        auto sv_to_int = [](std::string_view input) -> std::optional<int> {
            int out{};
            auto result = std::from_chars(input.data(), input.data() + input.size(), out);

            if (result.ec == std::errc::invalid_argument || result.ec == std::errc::result_out_of_range)
                return std::nullopt;

            return out;
        };

        for (auto&& str : input | std::views::split('.'))
        {
            // 应该不会遇到这个情况
            if (_count >= 4)
                break;

#if defined(__GNUC__) && __GNUC__ < 12
            const std::string_view token(&*str.begin(), std::ranges::distance(str));
#else
            const std::string_view token(str.begin(), str.end());
#endif

            _numbers[_count] = sv_to_int(token).value_or(0);
            _count++;
        }
    }

    bool operator==(const Version& other) const { return _numbers == other._numbers; }

    bool operator!=(const Version& other) const { return !(*this == other); }

    bool operator<(const Version& other) const
    {
        return std::ranges::lexicographical_compare(_numbers, other._numbers);
    }

    bool operator<=(const Version& other) const { return !(*this > other); }

    bool operator>(const Version& other) const { return other < *this; }

    bool operator>=(const Version& other) const { return !(*this < other); }

private:
    std::array<int, 4> _numbers{};
    int _count{};
};

// TODO: Make it configurable.
std::string FindDotnetRuntime()
{
    std::vector<std::filesystem::path> searchPaths{"../dotnet/host/fxr/"};
#ifdef WIN32
    std::string dll = "hostfxr.dll";
    searchPaths.emplace_back(R"(C:\Program Files\dotnet\host\fxr)");
#else
    std::string dll = "libhostfxr.so";
    searchPaths.emplace_back("/usr/share/dotnet/host/fxr/");
    searchPaths.emplace_back("/usr/lib/dotnet/host/fxr/");
#endif

    std::filesystem::path latest_file;
    Version latest_file_version;

    for (auto&& searchPath : searchPaths)
    {
        // ReSharper disable once CppRedundantQualifier
        if (!std::filesystem::exists(searchPath))
            continue;

        for (const auto& entry : std::filesystem::recursive_directory_iterator(searchPath))
        {
            if (entry.path().filename() != dll)
                continue;

            if (Version version(entry.path().parent_path().filename().string()); version > latest_file_version)
            {
                latest_file_version = version;
                latest_file = entry;
            }
        }
    }

    return latest_file.string();
}

bool LoadHostFxr()
{
    const auto buffer = FindDotnetRuntime();

#ifdef _WIN32
    if (buffer.empty())
    {
        __debugbreak();
    }
#endif

    void* lib = LoadLibrary(buffer.c_str());
    g_HostFxrUtils.Init =
        static_cast<hostfxr_initialize_for_runtime_config_fn>(GetExport(lib, "hostfxr_initialize_for_runtime_config"));
    g_HostFxrUtils.GetDelegate =
        static_cast<hostfxr_get_runtime_delegate_fn>(GetExport(lib, "hostfxr_get_runtime_delegate"));
    g_HostFxrUtils.Close = static_cast<hostfxr_close_fn>(GetExport(lib, "hostfxr_close"));

    return (g_HostFxrUtils.Init && g_HostFxrUtils.GetDelegate && g_HostFxrUtils.Close);
}

load_assembly_and_get_function_pointer_fn get_dotnet_load_assembly(const char_t* config_path)
{
    // Load .NET Core
    void* result = nullptr;
    hostfxr_handle cxt = nullptr;
    int rc = g_HostFxrUtils.Init(config_path, nullptr, &cxt);
    if (rc != 0 || cxt == nullptr)
    {
        std::cerr << "Init failed: " << std::hex << std::showbase << rc << std::endl;
        g_HostFxrUtils.Close(cxt);
        return nullptr;
    }

    // Get the load assembly function pointer
    rc = g_HostFxrUtils.GetDelegate(cxt, hdt_load_assembly_and_get_function_pointer, &result);

    if (rc != 0 || result == nullptr)
        std::cerr << "Get delegate failed: " << std::hex << std::showbase << rc << std::endl;

    // s_HostFxrUtils.Close(cxt);
    return static_cast<load_assembly_and_get_function_pointer_fn>(result);
}

static load_assembly_and_get_function_pointer_fn load_assembly_and_get_function_pointer = nullptr;

#ifdef _WIN32
std::wstring widen(const std::string& in)
{
    std::wstring out{};

    if (in.length() > 0)
    {
        // Calculate target buffer size (not including the zero terminator).
        const auto len =
            MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, in.c_str(), static_cast<int>(in.size()), nullptr, 0);
        if (len == 0)
        {
            throw std::runtime_error("Invalid character sequence.");
        }

        out.resize(len);
        // No error checking. We already know, that the conversion will succeed.
        MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, in.c_str(), static_cast<int>(in.size()), out.data(),
                            static_cast<int>(out.size()));
        // Use out.data() in place of &out[0] for C++17
    }

    return out;
}
#endif

void* GetDotnetFunctionPointer(const char* typeName, const char* method)
{
    void* pFunc = nullptr;

    auto entry_dll_path =
#ifdef _WIN32
        widen(GetExecutablePath().parent_path().parent_path().append("runtime").append("Rift.Runtime.dll").string());
#else
        GetExecutablePath().parent_path().parent_path().append("runtime").append("Rift.Runtime.dll").string();
#endif

    int rc = load_assembly_and_get_function_pointer(entry_dll_path.c_str(),
#ifdef _WIN32
                                                    widen(typeName).c_str(), widen(method).c_str(),
#else
                                                    typeName, method,
#endif
                                                    UNMANAGEDCALLERSONLY_METHOD, nullptr, &pFunc);
    assert(rc == 0 && pFunc != nullptr && "Failure: load_assembly_and_get_function_pointer()");
    return pFunc;
}

template <typename T>
T GetDotnetFunctionPointer(const char* typeName, const char* method)
{
    return reinterpret_cast<T>(GetDotnetFunctionPointer(typeName, method));
}

void* GetManagedFunction(const char* name)
{
    std::string _name(name);
    auto methodPos = _name.find_last_of(".");
    auto assemblyName = _name.substr(0, methodPos);

    char target[512];
    snprintf(target, sizeof(target), "Rift.Runtime.%s, Rift.Runtime", assemblyName.c_str());
    char methodName[512];
    snprintf(methodName, sizeof(methodName), "%sExport", _name.substr(methodPos + 1).c_str());

    return GetDotnetFunctionPointer(target, methodName);
}

bool Init()
{
    if (!LoadHostFxr())
    {
        assert(false && "Failure: LoadHostFxr()");
    }
    auto runtime_config_path =
#if _WIN32
        widen(GetExecutablePath()
                  .parent_path()
                  .parent_path()
                  .append("runtime")
                  .append("Rift.Runtime.runtimeconfig.json")
                  .string());
#else
        GetExecutablePath()
            .parent_path()
            .parent_path()
            .append("runtime")
            .append("Rift.Runtime.runtimeconfig.json")
            .string();
#endif

    load_assembly_and_get_function_pointer = get_dotnet_load_assembly(runtime_config_path.c_str());
    assert(load_assembly_and_get_function_pointer != nullptr && "Failure: get_dotnet_load_assembly()");

    return true;
}

void Shutdown()
{
    using RiftRuntimeShutdownFn_t = void(CORECLR_DELEGATE_CALLTYPE*)();

    const auto sharp_shutdown =
        GetDotnetFunctionPointer<RiftRuntimeShutdownFn_t>("Rift.Runtime.Bootstrap, Rift.Runtime", "Shutdown");

    sharp_shutdown();
}

int Bootstrap(void* natives)
{
    using RiftRuntimeInitFn_t = int(CORECLR_DELEGATE_CALLTYPE*)(void*);

    const auto rift_init =
        GetDotnetFunctionPointer<RiftRuntimeInitFn_t>("Rift.Runtime.Bootstrap, Rift.Runtime", "Init");
    return rift_init(natives);
}

} // namespace rift::coreclr
