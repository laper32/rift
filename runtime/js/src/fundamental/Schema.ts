/**
 * 我们定义项目是基于section的
 * 
 * 所以我们需要一个TomlManifest来明确Rift.toml里可能会有哪些section
 */
interface TomlManifest {
    workspace?: TomlWorkspace;
    folder?: TomlFolder;
    project?: TomlProject;
    target?: TomlTarget;
    plugin?: TomlPlugin;
}

interface TomlWorkspace {
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

interface TomlFolder {
    name?: string;
    members: Array<string>;
    exclude?: Array<string>;
}

interface TomlProject {
    name: string;
    authors: string[];
    version: string;
    description?: string;
    plugins?: string;
    dependencies?: string;
    metadata?: string;

    // Members/Exclude不能和Target同时存在。
    members?: Array<string>;
    exclude?: Array<string>;
}

interface TomlTarget {
    name: string;
    type: string;
    plugins?: string;
    dependencies?: string;
    metadata?: string;
}


interface TomlPlugin {
    name: string;
    version: string;
    authors: Array<string>;
    description?: string;
    metadata?: string;
    dependencies?: string;
    entry?: string;
}