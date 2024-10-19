using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Rift.Runtime.Bridge;


[assembly: DisableRuntimeMarshalling]
namespace Rift.Runtime;

public static class Bootstrap
{
    [UnmanagedCallersOnly]
    private static bool Init(nint natives)
    {
        Console.WriteLine("Bootstrap.Init");
        Adapter.Init(natives);
        return true;
        //unsafe
        //{
        //    ref var nativeVec = ref Unsafe.AsRef<CUtlVector<CUtlString>>(natives.ToPointer());
        //    foreach (var str in nativeVec)
        //    {
        //        Console.WriteLine(str.Get());
        //    }

        //    return true;
        //}
    }

    [UnmanagedCallersOnly]
    private static void Shutdown()
    {

    }
}
