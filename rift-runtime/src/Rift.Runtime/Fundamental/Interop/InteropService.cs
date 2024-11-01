﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Rift.Runtime.API.Fundamental.Interop;
using Rift.Runtime.API.Fundamental.Tier1;

namespace Rift.Runtime.Fundamental.Interop;

[StructLayout(LayoutKind.Sequential)]
internal unsafe struct RuntimeNative
{
    public fixed sbyte Name[128];
    public nint Func;

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


internal static class InteropService
{
    internal static readonly Dictionary<string, nint> NativeFunctions = new (StringComparer.OrdinalIgnoreCase);


    public static unsafe bool Init(nint natives)
    {
        ref var nativeVec = ref Unsafe.AsRef<CUtlVector<Pointer<RuntimeNative>>>(natives.ToPointer());

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
        var name = item.NameString;

        try
        {
            const string asmNameSpace = "Rift.Runtime";

            var pos = name.LastIndexOf('.');

            var typeName = name[..pos]       ?? throw new Exception("Shutdown to parse native method namespace");
            var sMethod  = name[(pos + 1)..] ?? throw new Exception("Shutdown to parse native method name");

            var sType = $"{asmNameSpace}.Fundamental.Interop.Natives.{typeName}";

            var type = AppDomain.CurrentDomain.GetAssemblies()
                .Where(x => x.ToString().StartsWith(asmNameSpace))
                .Select(x => x.GetType(sType))
                .FirstOrDefault();

            if (type is null)
            {
                Console.WriteLine($"Shutdown to find type \"{sType}\"");

                return;
            }

            if (type.GetField($"_{sMethod}", BindingFlags.Static | BindingFlags.NonPublic) is not { } field)
            {
                Console.WriteLine($"Shutdown to find method \"{sMethod}\" in type \"{sType}\"");

                return;
            }

            field.SetValue(null, item.Func);

            NativeFunctions.TryAdd(name, item.Func);
        }
        catch (Exception ex)
        {
            Console.WriteLine("Shutdown to bind native.", ex);
        }
    }
}