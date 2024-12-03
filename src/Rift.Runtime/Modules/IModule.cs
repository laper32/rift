namespace Rift.Runtime.Modules;

public interface IModule
{
    bool OnLoad();
    void OnAllLoaded();
    void OnUnload();
}