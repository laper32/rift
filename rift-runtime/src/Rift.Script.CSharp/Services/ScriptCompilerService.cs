using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.CSharp.Scripting.Hosting;
using Microsoft.CodeAnalysis.Scripting;
using Microsoft.CodeAnalysis.Scripting.Hosting;

namespace Rift.Script.CSharp.Services;
public interface IScriptCompilerService
{
    
}

public class ScriptCompilerService : IScriptCompilerService
{
    public ScriptCompilerService()
    {
        // force Roslyn to use ReferenceManager for the first time
        Task.Run(() =>
        {
            CSharpScript.Create<object>("1", ScriptOptions.Default, typeof(CommandLineScriptGlobals), new InteractiveAssemblyLoader()).RunAsync(new CommandLineScriptGlobals(Console.Out, CSharpObjectFormatter.Instance)).GetAwaiter().GetResult();
        });
    }

    private readonly IEnumerable<string> _predefinedLibraries =
    [
        "System",
        "System.IO",
        "System.Collections.Generic",
        "System.Console",
        "System.Diagnostics",
        "System.Dynamic",
        "System.Linq",
        "System.Linq.Expressions",
        "System.Text",
        "System.Threading.Tasks"
    ];

    // see: https://github.com/dotnet/roslyn/issues/5501
    private IEnumerable<string> _suppressedDiagnosticIds = ["CS1701", "CS1702", "CS1705"];

    //public ScriptOptions MakeScriptOptions(IScriptContext)

    /*public virtual ScriptOptions CreateScriptOptions(ScriptContext context, IList<RuntimeDependency> runtimeDependencies)
       {
           var scriptMap = runtimeDependencies.ToDictionary(rdt => rdt.Name, rdt => rdt.Scripts);
           var opts = ScriptOptions.Default.AddImports(ImportedNamespaces)
               .WithSourceResolver(new NuGetSourceReferenceResolver(new SourceFileResolver(ImmutableArray<string>.Empty, context.WorkingDirectory), scriptMap))
               .WithMetadataResolver(new NuGetMetadataReferenceResolver(ScriptMetadataResolver.Default.WithBaseDirectory(context.WorkingDirectory)))
               .WithEmitDebugInformation(true)
               .WithLanguageVersion(LanguageVersion.Preview)
               .WithFileEncoding(context.Code.Encoding ?? Encoding.UTF8);
       
           // if the framework is not Core CLR, add GAC references
           if (!ScriptEnvironment.Default.IsNetCore)
           {
               opts = opts.AddReferences(
                   "System",
                   "System.Core",
                   "System.Data",
                   "System.Data.DataSetExtensions",
                   "System.Runtime",
                   "System.Xml",
                   "System.Xml.Linq",
                   "System.Net.Http",
                   "Microsoft.CSharp");
       
               // on *nix load netstandard
               if (!ScriptEnvironment.Default.IsWindows)
               {
                   var netstandard = Assembly.Load("netstandard");
                   if (netstandard != null)
                   {
                       opts = opts.AddReferences(MetadataReference.CreateFromFile(netstandard.Location));
                   }
               }
           }
       
           if (!string.IsNullOrWhiteSpace(context.FilePath))
           {
               opts = opts.WithFilePath(context.FilePath);
           }
       
           return opts;
       }
       */
}