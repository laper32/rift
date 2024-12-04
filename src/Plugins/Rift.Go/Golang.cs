﻿using Microsoft.Extensions.DependencyInjection;
using Rift.Generate.Services;
using Rift.Go.Services;
using Rift.Runtime.Interfaces;
using Rift.Runtime.Plugins;
using Rift.Runtime.Scripting;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
public class Golang : RiftPlugin
{
    private IGenerateService _generateService = null!;

    public override bool OnLoad()
    {
        //_generateService = InterfaceManager.GetRequiredInterface<IGenerateService>(1);
        var services = new ServiceCollection();
        services.AddSingleton(this);
        services.AddSingleton<GolangGenerateService>();

        var provider = services.BuildServiceProvider();
        provider.GetRequiredService<GolangGenerateService>();

        ScriptManager.AddLibrary("Rift.Go");
        ScriptManager.AddNamespace("Rift.Go.Scripting");
        return base.OnLoad();
    }

    public override void OnAllLoaded()
    {
        _generateService          =  InterfaceManager.GetRequiredInterface<IGenerateService>(1);
        _generateService.Generate += GolangGenerateService.PerformGolangGenerate;
    }

    public override void OnUnload()
    {
        _generateService.Generate -= GolangGenerateService.PerformGolangGenerate;
        ScriptManager.RemoveNamespace("Rift.Go.Scripting");
        ScriptManager.RemoveLibrary("Rift.Go");
    }
}