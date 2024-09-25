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

        export namespace onLoad {
            export function addListener(callback: Function) {
                ops.op_register_plugin_load_listener(callback)
            }
        }

        export namespace onUnload {
            export function addListener(callback: Function) {
                ops.op_register_plugin_unload_listener(callback)
            }
        }

        export namespace onAllLoaded {
            export function addListener(callback: Function) {
                ops.op_register_plugin_all_loaded_listener(callback)
            }
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

    export class TaskDescriptor {
        constructor(name: String) {
            this.name = name;
        }

        public setDescription(description: String) {
            this.description = description;
            return this;
        }

        public markAsCommand() {
            this.exportToClap = true;
            return this;
        }

        private name: String;
        private description?: String;
        private exportToClap: Boolean = false;
    }

    export namespace tasks {

        /**
         * Create a new task.
         * 
         * @param task Task descriptor.
         * @param predicate The function that will be executed when the task is invoked.
         */
        export function add(task: TaskDescriptor, predicate: Function) {
            ops.op_register_task(task, predicate)
        }

        /**
         * Implements a declared task.
         * 
         * @param name Task name, must be an existing one, cannot override a task which has been implemented 
         * (unless explicitly declares 'override' flag, will be provided in the future.).
         * @param predicate The function that will be executed when the task is invoked.
         */
        export function impl(name: String, predicate: Function) {
            ops.op_impl_task(name, predicate)
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