﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Rift.Runtime.API.Fundamental.Interop;

[StructLayout(LayoutKind.Sequential)]
public unsafe struct Pointer<T> where T : unmanaged
{
    public T* Value;

    public ref T AsRef()
        => ref Unsafe.AsRef<T>(Value);

    public static implicit operator Pointer<T>(T* value)
        => new() { Value = value };

    public static implicit operator Pointer<T>(nint value)
        => (T*)value;
}