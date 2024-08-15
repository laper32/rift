export namespace rift {
    export class Plugin {
        constructor(name: String, version: String) {
            this.name = name;
            this.version = version;
        }
        public name: String = "";
        public version: String = "";
        public authors: String[] = []; // from Rift.toml
        public description: String = ""; // from Rift.toml
        public url: String = ""; // from Rift.toml
    }
    export class PluginRef {
        constructor(name: String, from: String) {
            this.name = name;
            this.from = from;
        }
        public name: String = "";
        public from: String = "";
    }

}