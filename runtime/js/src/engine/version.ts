const ops = (globalThis as any).Deno.core.ops;

class RiftVersion {
    constructor() {

    }

    public get version() {
        return ops.op_rift_version();
    }
    public get versionFull() {
        return ops.op_rift_version_full();
    }
    public get gitHash() {
        return ops.op_rift_git_hash();
    }
    public get gitHashShort() {
        return ops.op_rift_git_hash_short();
    }
}

var instance: RiftVersion;

export function get() {
    if (!instance) {
        instance = new RiftVersion();
    }
    return instance;
}