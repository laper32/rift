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

# 项目说明

### 编译说明

考虑到本项目涉及到的语言，工具链等跨度比较大，这里贴出来一个样例编译脚本：

```py
import os
import subprocess
import shutil
class PushdContext:
    cwd = None; original_dir = None
    def __init__(self, dirname): self.cwd = os.path.realpath(dirname)
    def __enter__(self): self.original_dir = os.getcwd(); os.chdir(self.cwd); return self
    def __exit__(self, type, value, tb): os.chdir(self.original_dir)

def pushd(dirname): return PushdContext(dirname)

def get_vs():
    cmdlet = ['C:\\Program Files (x86)\\Microsoft Visual Studio\\Installer\\vswhere.exe', '-latest', '-products', '*', '-requires', 'Microsoft.VisualStudio.Component.VC.Tools.x86.x64', '-property', 'installationPath']
    cmd = subprocess.run(cmdlet, stdout=subprocess.PIPE)
    if cmd.returncode != 0:
        raise RuntimeError('Failed to get visual studio path')
    return cmd.stdout.decode('utf-8').strip()

def get_vcvarsall():
    vs_path = get_vs()
    return os.path.join(vs_path, 'VC', 'Auxiliary', 'Build', 'vcvarsall.bat')

def configure_vcvarsall() -> dict[str, str]:
    import sys
    python = sys.executable
    process = subprocess.Popen('("%s" %s>nul)&&"%s" -c "import os; print(repr(os.environ))"' % (get_vcvarsall(), "x64", python), stdout=subprocess.PIPE, shell=True)
    stdout, _ = process.communicate()
    exitcode = process.wait()
    if exitcode != 0:
        raise RuntimeError('Failed to configure vcvarsall')
    result = None
    result = eval(stdout.decode('ascii').strip('environ'))
    if not result:
        raise Exception('Couldn\'t find/process vcvars32 batch file' )
    return result

def compile_engine():
    with pushd('D:/workshop/projects/rift/rift-engine') as _:
        cmdlet = ['cmake', '-G Visual Studio 17 2022', '-A x64', '-B', 'D:/workshop/projects/rift/rift-engine/build', '-S', '.']
        cmd = subprocess.Popen(cmdlet, stdout=subprocess.PIPE, shell=True, env=configure_vcvarsall())
        stdout, _ = cmd.communicate()
        exitcode = cmd.returncode
        if exitcode != 0:
            raise RuntimeError('Failed to configure rift engine')
        print(stdout.decode('utf-8'))
        cmdlet = ['cmake', '--build', 'D:/workshop/projects/rift/rift-engine/build', '--config', 'Release']
        cmd = subprocess.Popen(cmdlet, stdout=subprocess.PIPE, shell=True, env=configure_vcvarsall())
        stdout, _ = cmd.communicate()
        exitcode = cmd.returncode
        if exitcode != 0:
            raise RuntimeError('Failed to compile rift engine')
        print(stdout.decode('utf-8'))

def compile_cli():
    with pushd('D:/workshop/projects/rift/rift-cli') as _:
        cmdlet = ['cargo', 'build', '--release']
        environ = os.environ.copy()
        environ['RIFT_ENGINE_OUT_DIR']='D:/workshop/projects/rift/rift-engine/build/Release'
        cmd = subprocess.Popen(cmdlet, stdout=subprocess.PIPE, shell=True, env=environ)
        stdout, _ = cmd.communicate()
        exitcode = cmd.returncode
        if exitcode != 0:
            raise RuntimeError('Failed to compile rift cli')
        print(stdout.decode('utf-8'))

    pass

def compile_runtime():
    with pushd('D:/workshop/projects/rift/rift-runtime') as _:
        cmdlet = ['dotnet', 'publish', 'src/Rift.Runtime/Rift.Runtime.csproj', '-f','net8.0', '-r', 'win-x64', '--no-self-contained', '-c', 'Release', '--output', 'C:/Users/user/.rift/runtime']
        cmd = subprocess.run(cmdlet)
        if cmd.returncode != 0:
            raise RuntimeError('Failed to compile rift runtime')

def install_engine():
    if os.path.exists('C:/Users/user/.rift/bin/rift.engine.dll'):
        os.remove('C:/Users/user/.rift/bin/rift.engine.dll')
    with pushd ('D:/workshop/projects/rift/rift-engine/build/Release') as _: 
        shutil.copy('rift.engine.dll', 'C:/Users/user/.rift/bin/rift.engine.dll')

def install_cli():
    if os.path.exists('C:/Users/user/.rift/bin/rift.exe'):
        os.remove('C:/Users/user/.rift/bin/rift.exe')
    with pushd ('D:/workshop/projects/rift/rift-cli') as _: 
        shutil.copy('target/release/rift.exe', 'C:/Users/user/.rift/bin/rift.exe')

def compile():
    compile_runtime()
    compile_engine()
    compile_cli()

def install():
    install_engine()
    install_cli()

compile()
install()
```
