using Rift.Generate.Abstractions;
using Rift.Runtime.Collections.Generic;
using Rift.Runtime.Interfaces.Abstractions;
using Rift.Runtime.IO;
using Rift.Runtime.Plugins.Abstractions;
using Rift.Runtime.Plugins.Managers;

namespace Rift.Generate.Services;

public interface IGenerateService : IInterface
{
    void AddListener(RiftPlugin instance, IGenerateListener listener);
}

public class GenerateService : IGenerateService
{
    private static GenerateService _instance = null!;

    private readonly Dictionary<RiftPlugin, List<IGenerateListener>> _listeners = [];

    public uint InterfaceVersion => 1;

    public GenerateService()
    {
        _instance = this;
    }

    internal static void Execute()
    {
        GetInstance().GetListeners().ForEach((plugin, list) =>
        {
            if (PluginManager.GetPluginRuntimeInfo(plugin) is not { } rt)
            {
                return;
            }

            list.ForEachCatchException(x =>
            {
                x.OnGenerate();
            }, exception => Tty.Error(exception));

        });
    }

    public void AddListener(RiftPlugin instance, IGenerateListener listener)
    {
        // 如果没这个插件的话，插入这个插件的kv
        if (!_listeners.TryGetValue(instance, out var value))
        {
            value = [];
            _listeners.Add(instance, value);
        }

        // 只有这个列表里面真的没有这个listener才会注册上
        if (value.Find(x => x == listener) is null)
        {
            value.Add(listener);
        }

        //if (value.Find(x => x == listener) is null)
        //{
        //    value.Add(listener);
        //}
    }

    private Dictionary<RiftPlugin, List<IGenerateListener>> GetListeners() => _listeners;
    private static GenerateService GetInstance() => _instance;
}