import os, subprocess, shutil
from pathlib import Path

def construct_redistributable_dirs():
    if os.path.exists(".dist"): shutil.rmtree(".dist")
    if not os.path.exists(".dist"): os.makedirs(".dist")

    if not os.path.exists(".dist/bin"): os.makedirs(".dist/bin")
    if not os.path.exists(".dist/modules"): os.makedirs(".dist/modules")
    if not os.path.exists(".dist/plugins"): os.makedirs(".dist/plugins")
    pass

def build(release: bool):
    cmdlet = ['cargo', 'build']
    if release: cmdlet.append('--release')
    ret = subprocess.run(cmdlet)
    if ret.returncode != 0:
        print("Failed to build the project.")
        exit(1)

def move_output_to_redistributable(release: bool):
    if release: bin_path = "target/release/"
    else: bin_path = "target/debug/"
    engine_path = bin_path + "engine"
    if os.name == 'nt':
        engine_path += ".dll"
    rift_path = bin_path + "rift"
    if os.name == 'nt':
        rift_path += ".exe"
    
    target_engine_path = ".dist/bin"
    if os.name == 'nt':
        target_engine_path += "/engine.dll"
    target_rift_path = ".dist/bin"
    if os.name == 'nt':
        target_rift_path += "/rift.exe"

    if os.path.exists(engine_path):
        os.rename(engine_path, target_engine_path)
    if os.path.exists(rift_path):
        os.rename(rift_path, target_rift_path)
    
    os.makedirs(".dist/plugins/sample")
    with open(".dist/plugins/sample/Rift.toml", "w") as f:
        f.write("""[plugin]
name = "Sample Plugin"
version = "0.1.0"
author = ["rift-dev"]
description = "Sample Plugin"
url = "Sample Plugin"
dependencies = "rift/dependencies.ts"
metadata = "rift/metadata.ts"
                """)

    pass

def run_test():
    path = os.environ["PATH"]
    path += ";.dist/bin"
    os.environ["PATH"] = path
    cmdlet = ['rift']
    ret = subprocess.run(cmdlet)
    if ret.returncode != 0: exit(1)

if __name__ == '__main__':
    construct_redistributable_dirs()
    build(True)
    move_output_to_redistributable(True)
    run_test()

