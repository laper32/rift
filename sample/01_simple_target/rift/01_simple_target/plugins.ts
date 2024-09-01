
import { rift } from "../../../shared/plugin.ts";
import * as util from "./util.ts";
import * as cxx from "rift:cxx";

console.log("rift/01_simple_target/plugins.ts invoked");
util.call();
cxx;

[
    new rift.Plugin("cxx", "1.0.0"),
    new rift.Plugin("build", "1.0.0")
]

