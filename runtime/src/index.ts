// const core = (globalThis as any).Deno.core;
const ops = (globalThis as any).Deno.core.ops;

export namespace rift {
    export class Plugin {
        constructor(name: String) {
            this.name = name;
        }
        public setVersion(version: String) {
            this.version = version;
            return this;
        }
        private version: String = "";
        private name: String = "";
    }
    export function getHomePath() {
        return ops.get_home_path();
    }
    export function getInstallationPath() {
        return ops.get_installation_path();
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
            // ops.op_add_manifest_metadata({`${}`: value})
        }
    }


    export function getRiftPath() {
        return ops.get_rift_exe();
    }

}


/* 
const { core } = Deno;
const { ops } = core;

function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
}

const console = {
    log: (...args) => {
        core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
        core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
};

const runjs = {
    readFile: (path) => {
        return ops.op_read_file(path);
    },
    writeFile: (path, contents) => {
        return ops.op_write_file(path, contents);
    },
    removeFile: (path) => {
        return ops.op_remove_file(path);
    },

    fetch: async (url) => {
        return ops.op_fetch(url);
    },
};

var rift;
(function (rift) {
    class Plugin {
        constructor(name) {
            this.version = "";
            this.name = "";
            this.name = name;
        }
        setVersion(version) {
            this.version = version;
            return this;
        }
    }
    rift.Plugin = Plugin;
    function getHomePath() {
        return ops.get_home_path();
    }
    rift.getHomePath = getHomePath;
    function getInstallationPath() {
        return ops.get_installation_path();
    }
    rift.getInstallationPath = getInstallationPath;
    let plugins;
    (function (plugins) {
        function add(plugin) {
            ops.op_add_manifest_plugin(plugin);
        }
        plugins.add = add;
    })(plugins = rift.plugins || (rift.plugins = {}));
    let metadata;
    (function (metadata) {
        function add(key, value) {
            var kv = {};
            kv[`${key}`] = value;
            ops.op_add_manifest_metadata(kv);
            // ops.op_add_manifest_metadata({`${}`: value})
        }
        metadata.add = add;
    })(metadata = rift.metadata || (rift.metadata = {}));
    function getRiftPath() {
        return ops.get_rift_exe();
    }
    rift.getRiftPath = getRiftPath;
})(rift || (rift = {}));

globalThis.setTimeout = (callback, delay) => {
    ops.op_set_timeout(delay).then(callback);
};
globalThis.console = console;
globalThis.runjs = runjs;
*/