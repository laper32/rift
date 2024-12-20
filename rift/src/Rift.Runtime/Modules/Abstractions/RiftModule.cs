namespace Rift.Runtime.Modules.Abstractions;


/// <summary>
/// Abstract base class for modules. <br />
///
/// All modules must be derived from this class.
/// </summary>
public abstract class RiftModule : IModule
{
    /// <inheritdoc />
    public virtual bool OnLoad()
    {
        return true;
    }

    /// <inheritdoc />
    public virtual void OnAllLoaded()
    {
    }

    /// <inheritdoc />
    public virtual void OnUnload()
    {
    }
}
