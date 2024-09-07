// // const core = (globalThis as any).Deno.core;
const ops = (globalThis as any).Deno.core.ops;
namespace rift {
    export function getHomePath() {
        return ops.get_home_path();
    }
    export function getInstallationPath() {
        return ops.get_installation_path();
    }

    export function getRiftPath() {
        return ops.get_rift_exe();
    }
    class Plugin {
        constructor(name: String) {
            this.name = name;
        }
        public setVersion(version: String) {
            this.version = version;
            return this;
        }
        private version?: String;
        private name: String;
    }
}

// export namespace rift {
//     // 所有的依赖项都要从这里继承。
//     export class Plugin {
//         constructor(name: String) {
//             this.name = name;
//         }
//         public setVersion(version: String) {
//             this.version = version;
//             return this;
//         }
//         private version?: String;
//         private name: String;
//     }


//     export namespace plugins {
//         export function add(plugin: Plugin) {
//             ops.op_add_manifest_plugin(plugin)
//         }
//     }

//     export namespace dependencies {
//         export function add(dependency: any) {
//             ops.op_add_manifest_dependency(dependency)
//         }
//     }

//     export namespace metadata {
//         export function add(key: String, value: String) {
//             var kv: { [id: string]: String; } = {};
//             kv[`${key}`] = value;

//             ops.op_add_manifest_metadata(kv);
//             // ops.op_add_manifest_metadata({`${}`: value})
//         }
//     }



// }
