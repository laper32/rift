// 这些import应当要直接集成进runtime-pre-import中，尽量不要所有脚本都import
import { rift } from "../../../shared/dependency.ts";
[
    new rift.Dependency("boost", "1.0.0"),
    new rift.Dependency("fmt", "1.0.0")
]