﻿using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Rift.Runtime.Tier1;
using Rift.Runtime.Types;

namespace Rift.Runtime.Bridge;

[StructLayout(LayoutKind.Sequential)]
public unsafe struct RuntimeNative
{
    public fixed sbyte Name[128];
    public       nint  Func;

    public string NameString
    {
        get
        {
            fixed (sbyte* ptr = Name)
            {
                return new string(ptr);
            }
        }
    }
}


internal static class Adapter
{
    public static unsafe bool Init(nint natives)
    {
        /*
         ref var nativeVec = ref Unsafe.AsRef<CUtlVector<Pointer<NativeItem>>>(natives.ToPointer());

        foreach (var native in nativeVec)
        {
            InstallNative(ref native.AsRef());
        }
         */

        ref var nativeVec = ref Unsafe.AsRef<CUtlVector<Pointer<RuntimeNative>>>(natives.ToPointer());

        Console.WriteLine($"len: {nativeVec.Count}");

        try
        {
            foreach (var native in nativeVec)
            {
                InstallNative(ref native.AsRef());
            }
        }
        catch (Exception e)
        {
            Console.WriteLine(e);
        }

        
        return true;
    }

    private static void InstallNative(ref RuntimeNative item)
    {
        Console.WriteLine(item.NameString);
    }
}