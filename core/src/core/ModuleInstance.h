#ifndef RIFT_CORE_MODULEINSTANCE_H
#define RIFT_CORE_MODULEINSTANCE_H

#ifndef A
#    define A
#endif


namespace rift {
class IModule;

class ModuleInstance
{
public:
    bool Init();
    void Load();
    void Update();
    void Unload();

private:
    IModule *_module = nullptr;
};
} // namespace rift

#endif // !RIFT_CORE_MODULEINSTANCE_H
