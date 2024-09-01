// // 这些import应当要直接集成进runtime-pre-import中，尽量不要所有脚本都import
// import { rift } from "../../../shared/plugin";

// [
//     new rift.PluginRef("cxx", "project"), // From只能是Workspace, Project, Folder。因为本质上是树形关系。
//     new rift.PluginRef("build", "project")
// ]

import * as single_project from "rift:02_single_project/rift/02_single_project/plugins.ts";
single_project.call();