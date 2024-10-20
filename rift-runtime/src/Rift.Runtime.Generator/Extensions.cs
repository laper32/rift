﻿using Microsoft.CodeAnalysis.CSharp.Syntax;
using Microsoft.CodeAnalysis;

namespace Rift.Runtime.Generator;

public static class NamedTypeSymbolExtensions
{
    public static INamedTypeSymbol? GetFinalBaseType(this INamedTypeSymbol symbol)
    {
        var currentSymbol = symbol;
        var baseSymbol    = symbol.BaseType;

        while (baseSymbol is not null && baseSymbol.Name != "Object")
        {
            currentSymbol = baseSymbol;
            baseSymbol    = currentSymbol.BaseType;
        }

        return SymbolEqualityComparer.Default.Equals(currentSymbol, symbol) ? default : currentSymbol;
    }

    public static bool IsAssignableFrom(this INamedTypeSymbol symbol, string name)
    {
        if (symbol.Name == name)
            return true;

        if (symbol.AllInterfaces.Any(x => x.Name == name))
            return true;

        var baseType = symbol.BaseType;

        while (baseType is not null)
        {
            if (baseType.Name == name)
                return true;

            baseType = baseType.BaseType;
        }

        return false;
    }
}


public static class SyntaxNodeExtensions
{
    public static string GetNameSpace(this SyntaxNode syntax)
        => syntax.Parent switch
        {
            FileScopedNamespaceDeclarationSyntax fileScopedNamespaceDeclaration =>
                fileScopedNamespaceDeclaration.Name.ToString(),
            NamespaceDeclarationSyntax namespaceDeclarationSyntax => namespaceDeclarationSyntax.Name.ToString(),
            null                                                  => string.Empty, // or whatever you want to do
            var parentSyntax                                      => parentSyntax.GetNameSpace(),
        };
}

public class MethodSymbolParam
{
    public string Name           { get; set; } = null!;
    public string Type           { get; set; } = null!;
    public string UnmanagedType  { get; set; } = null!;
    public int    Utf8Size       { get; set; }
    public bool   IsNativeObject { get; set; }
    public bool   IsRef          { get; set; }
}

public class MethodSymbolParamContext
{
    public List<MethodSymbolParam> Params     { get; }
    public string                  ReturnType { get; }

    public MethodSymbolParamContext(List<MethodSymbolParam> @params, string returnType)
    {
        Params     = @params;
        ReturnType = returnType;
    }

    public string GetParametersString(string? prefixParam = null)
    {
        var @params = Params.Select(x => $"{x.Type} {x.Name}").ToList();

        if (prefixParam is not null)
        {
            @params.Insert(0, prefixParam);
        }

        return string.Join(", ", @params);
    }

    public string GetDelegateUnmanagedPointerString(string? prefixParam = null)
    {
        var @params = Params.Select(x => x.UnmanagedType).ToList();

        var @return = ReturnType switch
        {
            // "bool"                => "byte",
            "string" or "string?" => "sbyte*",
            _                     => ReturnType,
        };

        if (@return.StartsWith("System.Span<"))
        {
            @return = @return.Replace("System.Span", "Sharp.Shared.Types.NativeSpan");
        }

        if (prefixParam is not null)
        {
            @params.Insert(0, prefixParam);
        }

        var paramStr = string.Join(", ", @params);

        if (!string.IsNullOrEmpty(paramStr))
        {
            paramStr = $"{paramStr}, ";
        }

        return $"delegate* unmanaged<{paramStr}{@return}>";
    }

    public string GetCallDelegateUnmanagedPointerArgsString(string? prefixParam = null)
    {
        var args = Params.Select(x =>
                         {
                             var arg = x.Type switch
                             {
                                 "string" or "string?" => $"{x.Name}Ptr",

                                 // "bool"                  => $"(byte)({x.Name} ? 1 : 0)",
                                 _ when x.IsNativeObject => $"{x.Name}.GetAbsPtr()",
                                 _ when x.IsRef          => $"{x.Name}Ptr",
                                 _                       => x.Name,
                             };

                             return $"{arg}";
                         })
                         .ToList();

        if (prefixParam is not null)
        {
            args.Insert(0, prefixParam);
        }

        var argStr = string.Join(", ", args);

        return $"{argStr}";
    }

    public void BuildCallAndReturnDelegateUnmanagedPointer(CodeWriter builder, string methodName, string? prefixParam = null)
    {
        var returnStr = ReturnType switch
        {
            "void"   => "PLACEHOLDER;",
            "string" => "var __result = new string(PLACEHOLDER);",
            _        => "var __result = PLACEHOLDER;",
        };

        if (ReturnType != "void" && Params.Any(x => x.IsRef || x.Type is "string" or "string?"))
        {
            returnStr = ReturnType switch
            {
                "void"   => "PLACEHOLDER;",
                "string" => "__result = new string(PLACEHOLDER);",
                _        => "__result = PLACEHOLDER;",
            };

            builder.AppendLine($"{ReturnType} __result;");
        }

        var scopes = Params.Where(x => x.IsRef)
                           .Select(x => builder.BeginScope($"fixed ({x.UnmanagedType} {x.Name}Ptr = &{x.Name})"))
                           .ToList();

        scopes.AddRange(Params.Where(x => x.Type is "string" or "string?")
                              .Select(x => builder.BeginScope($"fixed ({x.UnmanagedType} {x.Name}Ptr = {x.Name}Bytes)")));

        builder.AppendLine(returnStr.Replace("PLACEHOLDER",
                                             $"{methodName}({GetCallDelegateUnmanagedPointerArgsString(prefixParam)})"));

        foreach (var disposable in scopes)
        {
            disposable.Dispose();
        }

        builder.AppendLine();

        // cleanupBytes
        foreach (var arg in Params.Where(x => x.Type is "string" or "string?"))
        {
            if (arg.Type == "string?")
            {
                using (builder.BeginScope($"if ({arg.Name}Bytes is not null)"))
                {
                    builder.AppendLine($"pool.Return({arg.Name}Bytes);");
                }
            }
            else
            {
                builder.AppendLine($"pool.Return({arg.Name}Bytes);");
            }
        }

        if (returnStr.Contains("__result"))
        {
            builder.AppendLine();
            builder.AppendLine("return __result;");
        }
    }

    public void BuildUtf8ConvertCode(CodeWriter builder)
    {
        var items = Params.Where(x => x.Type is "string" or "string?").ToList();

        if (items.Any())
        {
            builder.AppendLine("var pool = ArrayPool<byte>.Shared;");
        }

        foreach (var p in items)
        {
            if (p.Type is "string?")
            {
                builder.AppendLine($"byte[]? {p.Name}Bytes;");
            }
            else
            {
                builder.AppendLine($"byte[] {p.Name}Bytes;");
            }

            if (p.Type is "string?")
            {
                using (builder.BeginScope($"if ({p.Name} is null)"))
                {
                    builder.AppendLine($"{p.Name}Bytes = null;");
                }
            }

            using (p.Type is "string?" ? builder.BeginScope("else") : builder.BeginScope())
            {
                var byteLength = p.Utf8Size > 0 ? $"{p.Utf8Size}" : $"Encoding.UTF8.GetMaxByteCount({p.Name}.Length)";
                builder.AppendLine($"{p.Name}Bytes = pool.Rent({byteLength});");

                builder.AppendLine($"Utf8.FromUtf16({p.Name}, {p.Name}Bytes, out _, out var bytesWritten);");

                builder.AppendLine($"{p.Name}Bytes[bytesWritten] = 0;");
            }

            builder.AppendLine();
        }
    }
}


public static class MethodSymbolExtensions
{
    public static MethodSymbolParamContext GetParams(this IMethodSymbol symbol)
    {
        var items = symbol.Parameters.Select(x =>
                          {
                              var type     = x.Type.ToString();
                              var utf8Size = 0;

                              if (type is "string" or "string?")
                              {
                                  foreach (var attributeData in x.GetAttributes())
                                  {
                                      if (attributeData.AttributeClass?.Name == "NativeUtf8StringAttribute")
                                      {
                                          foreach (var arg in attributeData.ConstructorArguments)
                                          {
                                              utf8Size = (int) arg.Value!;
                                          }

                                          foreach (var arg in attributeData.NamedArguments)
                                          {
                                              if (arg.Key == "Size")
                                              {
                                                  utf8Size = (int) arg.Value.Value!;
                                              }
                                          }
                                      }
                                  }
                              }

                              var isNativeObject = x.Type is INamedTypeSymbol namedTypeSymbol
                                                   && namedTypeSymbol.IsAssignableFrom("INativeObject");

                              var isRef = x.RefKind > RefKind.None;

                              var unmanagedType = type switch
                              {
                                  "string" or "string?" => "byte*",

                                  // "bool"                => "byte",
                                  _ when isNativeObject => "nint",
                                  _                     => x.Type.ToString(),
                              };

                              if (unmanagedType.StartsWith("System.Span<"))
                              {
                                  unmanagedType = unmanagedType.Replace("System.Span", "Rift.Runtime.API.Fundamental.NativeSpan");
                              }

                              if (isRef)
                              {
                                  unmanagedType = $"{unmanagedType}*";
                                  type          = $"{x.RefKind.ToString().ToLower()} {type}";
                              }

                              return new MethodSymbolParam
                              {
                                  Name = x.Name switch
                                  {
                                      "event"  => "@event",
                                      "params" => "@params",
                                      _        => x.Name,
                                  },
                                  Type           = type,
                                  UnmanagedType  = unmanagedType,
                                  Utf8Size       = utf8Size,
                                  IsNativeObject = isNativeObject,
                                  IsRef          = isRef,
                              };
                          })
                          .ToList();

        return new MethodSymbolParamContext(items, symbol.ReturnType.ToString());
    }
}
