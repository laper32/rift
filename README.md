# Rift

注意：非常早期的阶段。很多设计还没稳定（A.K.A：随时都会有破坏性变动）

如果你对这个项目感兴趣，欢迎pr。

# 前言

## 为什么要叫Rift这个名字，而不是别的？

![alt text](assets/sc.png)<br/> 中文的话就是：<br/>
![alt text](assets/sc_cn.png)

## 为什么会有这个项目？

这个问题也等价于：我们既然已经有了形如CMake, Cargo, Gradle, Maven, Make,
Bazel等构建系统，为什么我们还要自己做一个？<br/>
嗯。。。如果你真的用过我上述说的这些构建系统，你就会发现这些构建系统各有各的问题。<br/>

- CMake自不必说，用过的都知道我在说什么；
- Cargo设计不错，但很可惜Rust Only。
- Gradle,
  Maven这两个其实也不错，但这两兄弟是JVM阵营的，尤其gradle现在开始推.kts，那就更加捆绑IDEA了。（VSCode可没有针对kotlin的官方支持）
- Make是Unix Only，Win想用你得装个Msys/Mingw/Cygwin，你确定你随时随地都能装？
- Bazel只要用过的都知道有多难用。。。

综上所述，思考上面构建系统的长处短处，针对自己需要的东西，自己设计一个构建系统咯。
