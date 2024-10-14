export const manifestIdentifier = "Rift.toml";

class VirtualManifest {
    private _data: FolderManifest | WorkspaceManifest;
    constructor(data: FolderManifest | WorkspaceManifest) {
        this._data = data;
    }
    get name() {
        return this._data.name;
    }
    get members() {
        return this._data.members;
    }
    get exclude() {
        return this._data.exclude;
    }
    get metadata() {
        var workspaceManifest = this._data as WorkspaceManifest;
        return workspaceManifest.metadata;
    }
    get plugins() {
        var workspaceManifest = this._data as WorkspaceManifest;
        return workspaceManifest.plugins;
    }
    get dependencies() {
        var workspaceManifest = this._data as WorkspaceManifest;
        return workspaceManifest.dependencies;
    }

}

class Manifest {
    private _data: ProjectManifest | TargetManifest;
    constructor(data: ProjectManifest | TargetManifest) {
        this._data = data;
    }

    get name() {
        return this._data.name;
    }

    get dependencies() {
        return this._data.dependencies;
    }
    get plugins() {
        return this._data.plugins;
    }
    get metadata() {
        return this._data.metadata;
    }
}

interface TargetManifest {
    name: string;
    type: string;
    plugins?: string;
    dependencies?: string;
    metadata?: string;
}

interface ProjectManifest {
    name: string;
    authors: Array<string>;
    version: string;
    description?: string;
    plugins?: string;
    dependencies?: string;
    metadata?: string;

    /**
     * 如果project和target同时存在，那么members和exclude将无法使用，就算里面写东西也会被忽略
     * 
     * 除此之外无限制。
     */
    members?: Array<string>;
        
    /**
     * 如果project和target同时存在，那么members和exclude将无法使用，就算里面写东西也会被忽略
     * 
     * 除此之外无限制。
     */
    exclude?: Array<string>;

    /**
     * 当且仅当只有一个target出现的时候才会有这个field
     */
    target?: TargetManifest;
}

interface FolderManifest {
    name?: string;
    members?: Array<string>;
    exclude?: Array<string>;
}

interface WorkspaceManifest {
    /**
     * 如果没有指定则会根据文件夹路径的情况生成一个
     */
    name?: string;
    /**
     * 没啥说的
     * 
     * 强制加，不能没有这个field，否则报错。
     */
    members: Array<string>;

    exclude?: Array<string>;

    /**
     * 文件路径
     */
    metadata?: string;

    /**
     * 文件路径
     */
    plugins?: string;

    /**
     * 文件路径
     */
    dependencies?: string;
}

interface PluginManifest {
    name: string;
    version: string;
    authors: Array<string>;
    description?: string;
    metadata?: string;
    dependencies?: string;

    /**
     * index.ts或者是用户自己定义的入口
     */
    entry: string;
}