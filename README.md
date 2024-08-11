# Rift

## 项目说明

Windows用户：由于我们使用V8，而rusty_v8有一个操作是创建符号链接，而创建符号链接需要管理员权限。

换句话说，如果你想参与Rift的开发，（至少目前而言）你将不可避免的必须使用管理员模式进行Rift的开发工作，包括但不限于: VSCode，Idea，cargo build。

除非rusty_v8上游针对这个特殊情况做了特殊处理，否则我们无法在非管理员模式下进行任何的开发工作

参见Issue: https://github.com/denoland/rusty_v8/issues/1563

如果你在网络上有一些问题，为了保证你能顺利编译运行Rift，你还需要做如下额外工作：

下载预编译好的rusty_v8。参见：https://github.com/denoland/rusty_v8/releases
放置文件你还需要做如下工作：
1. 根据版本号建立文件夹，比如说当前版本是v0.102.0，那么你需要建立v0.102.0文件夹
2. 把下载好的压缩包放入这个版本号文件夹中。

最终的话，大概长这样：
`C:/Users/user/.cache/rusty_v8/v0.102.0/rusty_v8_release_x86_64-pc-windows-msvc.lib.gz`
> 这里以我个人电脑来做示例。

添加环境变量：`{Key = RUSTY_V8_MIRROR, Value = "C:/Users/user/.cache/rusty_v8" }`

接下来，重启你的开发环境（VSCode, Idea, Terminal等，Windows环境下必须是管理员模式），接下来就可以正常开发了。