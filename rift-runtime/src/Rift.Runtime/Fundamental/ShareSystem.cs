// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Plugin;

namespace Rift.Runtime.Fundamental;

internal interface IShareSystemInternal : IShareSystem;

internal class ShareSystem : IShareSystemInternal, IInitializable
{
    private readonly ILogger<ShareSystem> _logger;
    public ShareSystem()
    {
        IShareSystem.Instance = this;
        _logger = IRuntime.Instance.Logger.CreateLogger<ShareSystem>();
    }


    public void AddInterface(ISharable @interface, IPlugin plugin)
    {
        throw new NotImplementedException();
    }

    public T? GetInterface<T>(uint version) where T : class, ISharable
    {
        throw new NotImplementedException();
    }

    public bool HasInterface<T>() where T : class, ISharable
    {
        throw new NotImplementedException();
    }

    public T GetRequiredInterface<T>(uint version) where T : class, ISharable
    {
        throw new NotImplementedException();
    }

    public bool TryGetInterface<T>(string name, uint version, out T ret) where T : class, ISharable
    {
        throw new NotImplementedException();
    }

    public bool Init()
    {

        return true;
    }

    public void Shutdown()
    {

    }
}