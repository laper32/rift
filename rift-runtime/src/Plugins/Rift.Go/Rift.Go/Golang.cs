using Rift.Generate.API;
using Rift.Go.API;
using Rift.Runtime.API.Plugin;

namespace Rift.Go;

// ReSharper disable once UnusedMember.Global
public class Golang : RiftPlugin
{
    void Call()
    {
        Example.Call1();
        ExampleGo.Call2();
    }
}