using System.Runtime.InteropServices;

namespace Rift.Runtime.API.Fundamental.Interop;

public class NativeString
{
    public static unsafe string ReadFromPointer(sbyte* ptr)
        => Marshal.PtrToStringUTF8((nint) ptr) ?? string.Empty;
}