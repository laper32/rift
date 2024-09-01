// // 这些import应当要直接集成进runtime-pre-import中，尽量不要所有脚本都import
// import { rift } from "../../../shared/dependency";
// [
//     new rift.DependencyRef("boost", "project"), // From只能是Workspace, Project, Folder。因为本质上是树形关系。
//     new rift.DependencyRef("fmt", "project")
// ]