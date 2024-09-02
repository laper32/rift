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

const rift = {
    internal: {
        getRiftExePath: () => {
            return ops.get_rift_exe();
        },
        getHomePath: () => {
            return ops.get_home_path();
        },
        getInstallationPath: () => {
            return ops.get_installation_path();
        },
    },

    getRiftExePath: () => {
        return rift.internal.getRiftExePath();
    },
    getHomePath: () => {
        return rift.internal.getHomePath();
    },
    getInstallationPath: () => {
        return rift.internal.getInstallationPath();
    },
}


globalThis.setTimeout = (callback, delay) => {
    ops.op_set_timeout(delay).then(callback);
};
globalThis.console = console;
globalThis.runjs = runjs;