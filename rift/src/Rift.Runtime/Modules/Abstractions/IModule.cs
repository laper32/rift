namespace Rift.Runtime.Modules.Abstractions;

/// <summary>
///     Interface for module operations.
/// </summary>
public interface IModule
{
    /// <summary>
    ///     Called when the module is loaded.
    /// </summary>
    /// <returns> True if the module loaded successfully, otherwise false. </returns>
    bool OnLoad();

    /// <summary>
    ///     Called when all modules are loaded.
    /// </summary>
    void OnAllLoaded();

    /// <summary>
    ///     Called when the module is unloaded.
    /// </summary>
    void OnUnload();
}