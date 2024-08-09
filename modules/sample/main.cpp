struct RiftModuleVersion
{
    int major;
    int minor;
    int patch;
};

struct RiftModuleDescriptor
{
    char name[256];
    struct RiftModuleVersion version;
    char description[4096];
    char author[256];
    char url[256];
};

struct RiftModule
{
    bool (*OnLoad)(void);
    void (*OnUnload)(void);
    void (*OnAllLoad)(void);
    struct RiftModuleDescriptor descriptor;
};
extern "C"
{
    void RegisterRiftModule(const struct RiftModule *module);
}

#include <memory>

std::unique_ptr<RiftModule> g_Module = std::make_unique<RiftModule>();

bool OnLoad()
{
    return true;
}

void OnUnload()
{
}

void OnAllLoad()
{
}

extern "C" __declspec(dllexport) int ModuleMain()
{
    g_Module->OnLoad = &OnLoad;
    g_Module->OnAllLoad = &OnAllLoad;
    g_Module->OnUnload = &OnUnload;
    g_Module->descriptor = RiftModuleDescriptor{"Sample", {1, 0, 0}, "Sample module", "rift-dev", ""};
    RegisterRiftModule(g_Module.get());

    return 0;
}