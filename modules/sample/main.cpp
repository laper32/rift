struct RiftModuleVersion
{
    int major;
    int minor;
    int patch;
};

struct RiftModuleDescriptor
{
    const char *name;
    struct RiftModuleVersion version;
    const char *description;
    const char *author;
    const char *url;
};

struct RiftModule
{
    bool (*OnLoad)(void);
    void (*OnUnload)(void);
    void (*OnAllLoad)(void);
    const struct RiftModuleDescriptor *descriptor;
};
extern "C"
{
    void RegisterRiftModule(const struct RiftModule *module);
    const struct RiftModuleDescriptor *DeclareRiftModuleDescriptor(const char *name,
                                                                   struct RiftModuleVersion version,
                                                                   const char *description,
                                                                   const char *author,
                                                                   const char *url);
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
    g_Module->OnUnload = &OnUnload;
    g_Module->OnUnload = &OnUnload;
    g_Module->descriptor = DeclareRiftModuleDescriptor("Sample", {1, 0, 0}, "Sample module", "rift-dev", "");
    RegisterRiftModule(g_Module.get());

    return 0;
}