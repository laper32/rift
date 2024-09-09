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
    export class Plugin {
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
    export namespace plugins {
        export function add(plugin: Plugin) {
            ops.op_add_manifest_plugin(plugin)
        }
    }
    export namespace metadata {
        export function add(key: String, value: String) {
            var kv: { [id: string]: String; } = {};
            kv[`${key}`] = value;
            ops.op_add_manifest_metadata(kv);
        }
    }
    export interface IDependency {

    }
    export namespace dependencies {
        export function add(dependency: IDependency) {
            ops.op_add_manifest_dependencies(dependency)
        }
    }
}

class CxxDependency implements rift.IDependency {
    private name: String;
    constructor(name: String) {
        this.name = name;
    }

    private version?: String;
    public setVersion(version: String) {
        this.version = version;
        return this;
    }

    private path?: String;
    public setPath(path: String) {
        this.path = path;
        return this;
    }

    private git?: String;
    public setGitUrl(git: String) {
        this.git = git;
        return this;
    }

    private gitCommit?: String;
    public setGitCommit(commit: String) {
        if (!this.git) {
            throw new Error("git url is not set");
        }
        this.gitCommit = commit;
        return this;
    }
}