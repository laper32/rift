// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.Plugin;

namespace Rift.Runtime.Fundamental;


internal class ShareSystemInternal : ShareSystem, IInitializable
{
    public new static ShareSystemInternal          Instance { get; private set; } = null!;

    private readonly  ILogger<ShareSystemInternal> _logger;
    public ShareSystemInternal()
    {
        Instance = this;
        _logger = RuntimeInternal.Instance.Logger.CreateLogger<ShareSystemInternal>();
    }

    public override void AddInterface(ISharable @interface, IPlugin plugin)
    {
        throw new NotImplementedException();
    }

    public override T? GetInterface<T>(uint version) where T : class
    {
        throw new NotImplementedException();
    }

    public override bool HasInterface<T>() 
    {
        throw new NotImplementedException();
    }

    public override T GetRequiredInterface<T>(uint version)
    {
        throw new NotImplementedException();
    }

    public override bool TryGetInterface<T>(string name, uint version, out T ret)
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