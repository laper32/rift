# 快速上手

在进行下文的介绍之前，将提供如下前置信息：

1. 我们的目标语言是Go。
2. Go插件依赖`Rift.Generate`，一个用于生成目标编程语言配置文件的插件。其提供`generate`命令。

如何初始化一个项目：

所有的基本信息都在下方的注释中说明。

```toml
[project] # 该标签用来说明此为一个项目
name = "example-project" # 项目名，必填
version = "0.1.0" # 项目版本，必填
authors = [""] # 项目作者，必填
description = "" # 项目描述，选填
plugins = "rift/plugins.csx" # 插件路径，起始路径为Rift.toml所在的文件夹
configure = "rift/configure.csx" # 配置路径，起始路径为Rift.toml所在的文件夹
dependencies = "rift/dependencies.csx" # 依赖路径，起始路径为Rift.toml所在的文件夹

[target] # 该标签用于说明具体的单元
name = "02_single_target" # 单元名，必填
type = "bin" # 单元类型，必填
```

当你声明了`plugins`字段后，你只需在`rift/plugins.csx`文件中写入：

> 由于目前我的测试环境主要是Go，所以只提供了Go的支持，其他的还在新建文件夹，见谅。

> 此外，在声明插件时，只能使用`Rift.Runtime`提供的函数，插件扩展全部无法使用。

```cs
Plugins.Add([
    new PackageReference("Rift.Go")
]);
```

当你声明插件后，此时插件已经加载，你可以在`configure`和`dependencies`中使用插件提供的扩展函数。

当我们声明了`configure`后，我们可进行如下操作：

```cs
//rift/configure.csx
using Rift.Generate;

// set GOPROXY=https://goproxy.cn,direct

Environment.SetEnvironmentVariable("GOPROXY", "https://goproxy.cn,direct");

Package.Configure(config =>
{
    config.GolangVersion("1.22.2");
});

// rift hello
Tasks.Register("rift.hello", (configure) =>
{
    configure
        .SetDeferException(true)
        .SetIsCommand(true)
        ;
});

// rift hello nest
Tasks.Register("rift.hello.nest", configure =>
{
    configure.SetIsCommand(true);
});

Tasks.Register("this.will.trigger.command.warning", configure =>
{
    configure.SetIsCommand(true);
});
Sth.Call();
```

`dependencies`也进行一样的操作。

```cs
// rift/dependencies.csx
Dependencies.Add([
    new PackageReference("github.com/laper32/goose"),
]);
```

可以看到，我们加载了插件`Rift.Go`，依赖`github.com/laper32/goose`包。

运行`rift generate`，此时将会完成文件的生成。

此时的文件夹结构大致如下：

```
example-project
│  Rift.toml
│
└─subpath
    │  go.mod
    │  go.sum
    │  Rift.toml
    │
    └─rift
            configure.csx
            dependencies.csx
            plugins.csx
```

这时我们查看`go.mod`文件：

```
module example-project

go 1.22.2

require (
	github.com/laper32/goose v0.1.0
)
```

大功告成，接下来我们就可以愉快的开发了。
