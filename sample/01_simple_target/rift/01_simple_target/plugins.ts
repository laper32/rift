// 这些import应当要直接集成进runtime-pre-import中，尽量不要所有脚本都import
import { rift } from "../../../shared/plugin";

[
    new rift.Plugin("cxx", "1.0.0"),
    new rift.Plugin("build", "1.0.0")
]