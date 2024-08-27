import * as rift from "rift";

if (rift.hasPlugin("cxx")) {
    rift.addMetadata("cxx.cpp.standard", "17");
    rift.addMetadata("cxx.c.standard", "11")

    let inputArg = rift.getInputArg("CXX_COMPILER");
    if (inputArg != "") {
        rift.addMetadata("cxx.compiler", inputArg);
    } else {
        if (rift.isWindows()) {
            rift.addMetadata("cxx.compiler", "msvc");
        } else {
            rift.addMetadata("cxx.compiler", "gcc");
        }
    }

}