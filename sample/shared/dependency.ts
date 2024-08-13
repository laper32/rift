export namespace rift {
    export class Dependency {
        constructor(name: String, version: String) {
            this.name = name;
            this.version = version;
        }
        public name: String = "";
        public version: String = "";
    }

    export class DependencyRef {
        constructor(name: String, from: String) {
            this.name = name;
            this.from = from;
        }
        public name: String = "";
        public from: String = "";
    }
}