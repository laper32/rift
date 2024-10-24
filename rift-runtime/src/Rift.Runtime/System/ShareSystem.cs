// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Microsoft.Extensions.Logging;
using Rift.Runtime.API.Abstractions;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.API.System;

namespace Rift.Runtime.System;

internal interface IShareSystemInternal : IShareSystem, IInitializable;

internal class ShareSystem : IShareSystemInternal
{
    private bool _init;
    private bool _shutdown;
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
        _init = true;
        _shutdown = false;
        Console.WriteLine("ShareSystem.Init");
        return true;
    }

    public void Shutdown()
    {
        _shutdown = true;
        _init = false;
    }
}