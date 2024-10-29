using System.Reflection;
using System.Runtime.Loader;

namespace Rift.Script.CSharp.Fundamental;

/// <summary>
/// Represents assembly load context for a script with full and automatic assembly isolation.
/// </summary>
public class ScriptAssemblyLoadContext : AssemblyLoadContext
{
    /// <summary>
    /// Initializes a new instance of the <see cref="ScriptAssemblyLoadContext"/> class.
    /// </summary>
    public ScriptAssemblyLoadContext()
    {
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="ScriptAssemblyLoadContext"/> class
    /// with a name and a value that indicates whether unloading is enabled.
    /// </summary>
    /// <param name="name"><inheritdoc/></param>
    /// <param name="isCollectible"><inheritdoc/></param>
    public ScriptAssemblyLoadContext(string? name, bool isCollectible = false) :
        base(name, isCollectible)
    {
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="ScriptAssemblyLoadContext"/> class
    /// with a value that indicates whether unloading is enabled.
    /// </summary>
    /// <param name="isCollectible"><inheritdoc/></param>
    protected ScriptAssemblyLoadContext(bool isCollectible) :
        base(isCollectible)
    {
    }

    /// <inheritdoc/>
    protected override Assembly? Load(AssemblyName assemblyName) => InvokeLoading(assemblyName);

    /// <inheritdoc/>
    protected override IntPtr LoadUnmanagedDll(string unmanagedDllName) => InvokeLoadingUnmanagedDll(unmanagedDllName);

    /// <summary>
    /// Provides data for the <see cref="Loading"/> event.
    /// </summary>
    internal sealed class LoadingEventArgs(AssemblyName assemblyName) : EventArgs
    {
        public AssemblyName Name { get; } = assemblyName;
    }

    /// <summary>
    /// Represents a method that handles the <see cref="Loading"/> event.
    /// </summary>
    /// <param name="sender">The sender.</param>
    /// <param name="args">The arguments.</param>
    /// <returns>The loaded assembly or <c>null</c> if the assembly cannot be resolved.</returns>
    internal delegate Assembly? LoadingEventHandler(ScriptAssemblyLoadContext sender, LoadingEventArgs args);

    private LoadingEventHandler? _loadingEventHandler;

    /// <summary>
    /// Occurs when an assembly is being loaded.
    /// </summary>
    internal event LoadingEventHandler Loading
    {
        add => _loadingEventHandler += value;
        remove => _loadingEventHandler -= value;
    }

    private Assembly? InvokeLoading(AssemblyName assemblyName)
    {
        var eh = _loadingEventHandler;
        if (eh == null)
        {
            return null;
        }
        var args = new LoadingEventArgs(assemblyName);

        // ReSharper disable once LoopCanBeConvertedToQuery
        foreach (var @delegate in eh.GetInvocationList())
        {
            var handler  = (LoadingEventHandler)@delegate;
            var assembly = handler(this, args);
            if (assembly != null)
            {
                return assembly;
            }
        }
        return null;
    }

    /// <summary>
    /// Provides data for the <see cref="LoadingUnmanagedDll"/> event.
    /// </summary>
    internal sealed class LoadingUnmanagedDllEventArgs(
        string               unmanagedDllName,
        Func<string, IntPtr> loadUnmanagedDllFromPath)
        : EventArgs
    {
        public string               UnmanagedDllName         { get; } = unmanagedDllName;
        public Func<string, IntPtr> LoadUnmanagedDllFromPath { get; } = loadUnmanagedDllFromPath;
    }

    /// <summary>
    /// Represents a method that handles the <see cref="LoadingUnmanagedDll"/> event.
    /// </summary>
    /// <param name="sender">The sender.</param>
    /// <param name="args">The arguments.</param>
    /// <returns>The loaded DLL or <see cref="IntPtr.Zero"/> if the DLL cannot be resolved.</returns>
    internal delegate IntPtr LoadingUnmanagedDllEventHandler(ScriptAssemblyLoadContext sender, LoadingUnmanagedDllEventArgs args);

    private LoadingUnmanagedDllEventHandler? _loadingUnmanagedDllHandler;

    /// <summary>
    /// Occurs when an unmanaged DLL is being loaded.
    /// </summary>
    internal event LoadingUnmanagedDllEventHandler LoadingUnmanagedDll
    {
        add => _loadingUnmanagedDllHandler += value;
        remove => _loadingUnmanagedDllHandler -= value;
    }

    private IntPtr InvokeLoadingUnmanagedDll(string unmanagedDllName)
    {
        var eh = _loadingUnmanagedDllHandler;

        if (eh == null)
        {
            return IntPtr.Zero;
        }

        var args = new LoadingUnmanagedDllEventArgs(unmanagedDllName, LoadUnmanagedDllFromPath);
        foreach (var @delegate in eh.GetInvocationList())
        {
            var handler = (LoadingUnmanagedDllEventHandler)@delegate;
            var dll     = handler(this, args);
            if (dll != IntPtr.Zero)
            {
                return dll;
            }
        }
        return IntPtr.Zero;
    }
}