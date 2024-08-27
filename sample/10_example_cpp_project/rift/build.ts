import * as rift from 'rift';
import * as cxx from "cxx";

cxx.includeDirectories.push(`${rift.projectRoot}/include`);

var example_cpp = cxx.target("example-cpp");
example_cpp.includeDirectories.push(`${rift.projectRoot}/src/**`);
example_cpp.sourceFiles.push([
    `${rift.projectRoot}/src/main.cpp`,
    `${rift.projectRoot}/src/example.cpp`,
    `${rift.projectRoot}/src/example.cpp`,
    `${rift.projectRoot}/src/server/type/CBaseEntity.cpp`,
]);

var compileOptions = new Array();
if (rift.isWindows()) {
    if (cxx.getToolchain() == "msvc") {
        if (cxx.getCompiler() == "msvc") {
            compileOptions.push([
                "/std:c++17",
                "/EHsc",
                "/wd4819",
                "/wd4828",
                "/wd5033",
                "/wd4996"
            ]);
            cxx.compileOptions.push(compileOptions);
        }

        if (cxx.getCompiler() == "clang-cl") {
            compileOptions.push([
                "-std=c++17",
                "-fms-extensions",
                "-fms-compatibility",
                "-fms-compatibility-version=19.11",
                "-fmsc-version=1911",
                "-fno-exceptions",
                "-fno-rtti"
            ]);
            cxx.compileOptions.push(compileOptions);
        }
    }

    // Mingw/Msys/Msys2/CygWin
    if (cxx.getToolchain() == "mingw") {
        if (cxx.getCompiler() == "g++") {
            compileOptions.push([
                "-std=c11",
                "-fno-exceptions",
                "-fno-rtti"
            ]);
            cxx.compileOptions.push(compileOptions);
        }
        if (cxx.getCompiler() == "gcc") {
            compileOptions.push([
                "-std=c11",
                "-fno-exceptions",
                "-fno-rtti"
            ]);
            cxx.compileOptions.push(compileOptions);
        }

        if (cxx.getCompiler() == "clang") {
            compileOptions.push([
                "-std=c11",
                "-fno-exceptions",
                "-fno-rtti"
            ]);
            cxx.compileOptions.push(compileOptions);
        }
    }
}
