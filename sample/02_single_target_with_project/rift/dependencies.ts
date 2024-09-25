

class RustDependency implements rift.IDependency {
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

rift.dependencies.add(
    new CxxDependency("boost")
        .setGitUrl("git://github.com/boostorg/boost.git")
        .setGitCommit("master")
);
rift.dependencies.add(new RustDependency("clap").setVersion("2.33.0"));