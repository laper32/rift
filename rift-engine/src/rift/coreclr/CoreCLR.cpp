// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#include "rift/coreclr/CoreCLR.h"

#include "rift/coreclr/CoreCLRDelegates.h"
#include "rift/coreclr/Hostfxr.h"

#include "rift/fundamental/PrimitivePath.h"
#include "rift/fundamental/String.h"


#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif

#ifdef _WIN32
#include <windows.h>
#define STR(s) L##s // NOLINT(clang-diagnostic-unused-macros)
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

// ReSharper disable once IdentifierTypo
struct HostfxrUtils
{
    // Those below are all function pointer!

    // ReSharper disable once CppInconsistentNaming
    hostfxr_initialize_for_runtime_config_fn Init;
    // ReSharper disable once CppInconsistentNaming
    hostfxr_get_runtime_delegate_fn GetDelegate;
    // ReSharper disable once CppInconsistentNaming
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
    void* f = GetProcAddress(static_cast<HMODULE>(h), name);  // NOLINT(clang-diagnostic-microsoft-cast)
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
        auto sv_to_int = [](const std::string_view sv_input) -> std::optional<int> {
            int out{};

            if (const auto [ptr, ec] = std::from_chars(sv_input.data(), sv_input.data() + sv_input.size(), out);
                ec == std::errc::invalid_argument || ec == std::errc::result_out_of_range)
                return std::nullopt;

            return out;
        };

        for (auto&& str : input | std::views::split('.'))
        {
            // 应该不会遇到这个情况  // NOLINT(clang-diagnostic-invalid-utf8)
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

std::string FindDotnetRuntime()
{
    std::vector<std::filesystem::path> searchPaths{"../dotnet/host/fxr/"};
#ifdef WIN32
    // ReSharper disable once StringLiteralTypo
    std::string dll = "hostfxr.dll";
    searchPaths.emplace_back(R"(C:\Program Files\dotnet\host\fxr)");
#else
    std::string dll = "libhostfxr.so";
    searchPaths.emplace_back("/usr/share/dotnet/host/fxr/");
    searchPaths.emplace_back("/usr/lib/dotnet/host/fxr/");
#endif

    std::filesystem::path latest_file;
    Version latest_file_version;

    for (auto&& search_path : searchPaths)
    {
        // ReSharper disable once CppRedundantQualifier
        if (!std::filesystem::exists(search_path))
            continue;

        for (const auto& entry : std::filesystem::recursive_directory_iterator(search_path))
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
        // ReSharper disable once StringLiteralTypo
        static_cast<hostfxr_initialize_for_runtime_config_fn>( // NOLINT(clang-diagnostic-microsoft-cast)
            GetExport(lib, "hostfxr_initialize_for_runtime_config"));
    g_HostFxrUtils.GetDelegate =
        // ReSharper disable once StringLiteralTypo
        static_cast<hostfxr_get_runtime_delegate_fn>( // NOLINT(clang-diagnostic-microsoft-cast)
            GetExport(lib, "hostfxr_get_runtime_delegate"));
    // ReSharper disable once StringLiteralTypo
    g_HostFxrUtils.Close =
        static_cast<hostfxr_close_fn>(GetExport(lib, "hostfxr_close")); // NOLINT(clang-diagnostic-microsoft-cast)

    return (g_HostFxrUtils.Init && g_HostFxrUtils.GetDelegate && g_HostFxrUtils.Close);
}

load_assembly_and_get_function_pointer_fn GetDotnetLoadAssembly(const char_t* config_path)
{
    // Load .NET Core
    void* result = nullptr;
    hostfxr_handle cxt = nullptr;
    int rc = g_HostFxrUtils.Init(config_path, nullptr, &cxt);
    if (rc != 0 || cxt == nullptr)
    {
        std::cerr << "Init failed: " << std::hex << std::showbase << rc << '\n';
        g_HostFxrUtils.Close(cxt);
        return nullptr;
    }

    // Get the load assembly function pointer
    rc = g_HostFxrUtils.GetDelegate(cxt, hdt_load_assembly_and_get_function_pointer, &result);

    if (rc != 0 || result == nullptr)
        std::cerr << "Get delegate failed: " << std::hex << std::showbase << rc << '\n';

    return static_cast<load_assembly_and_get_function_pointer_fn>(result); // NOLINT(clang-diagnostic-microsoft-cast)
}

namespace {

load_assembly_and_get_function_pointer_fn g_LoadAssemblyAndGetFunctionPointer = nullptr;

}


void* GetDotnetFunctionPointer(const char* type_name, const char* method)
{
    void* ret = nullptr;

    const auto entry_dll_path =
#ifdef _WIN32
        Widen(GetExecutablePath().parent_path().parent_path().append("runtime").append("Rift.Runtime.dll").string());
#else
        GetExecutablePath().parent_path().parent_path().append("runtime").append("Rift.Runtime.dll").string();
#endif

    // ReSharper disable once CppDeclaratorNeverUsed
    int rc = g_LoadAssemblyAndGetFunctionPointer(entry_dll_path.c_str(),
#ifdef _WIN32
                                                 Widen(type_name).c_str(), Widen(method).c_str(),
#else
                                                 typeName, method,
#endif
                                                 UNMANAGEDCALLERSONLY_METHOD, nullptr, &ret);
    assert(rc == 0 && pFunc != nullptr && "Failure: load_assembly_and_get_function_pointer()");
    return ret;
}

template <typename T>
T GetDotnetFunctionPointer(const char* type_name, const char* method)
{
    return reinterpret_cast<T>(GetDotnetFunctionPointer(type_name, method));
}

void* GetManagedFunction(const char* fn_name)
{
    const std::string name(fn_name);
    const auto method_pos = name.find_last_of('.');
    const auto assembly_name = name.substr(0, method_pos);

    char target[512];
    snprintf(target, sizeof(target), "Rift.Runtime.%s, Rift.Runtime", assembly_name.c_str()); // NOLINT(cert-err33-c)
    char method_name[512];
    snprintf(method_name, sizeof(method_name), "%sExport", name.substr(method_pos + 1).c_str()); // NOLINT(cert-err33-c)

    return GetDotnetFunctionPointer(target, method_name);
}

bool Init()
{
    if (!LoadHostFxr())
    {
        assert(false && "Failure: LoadHostFxr()");
    }
    const auto runtime_config_path =
#if _WIN32
        Widen(GetExecutablePath()
                  .parent_path()
                  .parent_path()
                  .append("runtime")
                  // ReSharper disable once StringLiteralTypo
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

    g_LoadAssemblyAndGetFunctionPointer = GetDotnetLoadAssembly(runtime_config_path.c_str());
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
