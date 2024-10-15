import { isAbsolute } from "@std/path";
import * as fs from "@std/fs";

class WorkspaceManager {
    private value: number = 0;
    private currentManifest?: String;

    public call() {
        this.value++;
        console.log('WorkspaceManager call =>', this.value);
        isAbsolute("D:/workshop/projects/rift/runtime/js/src/managers/WorkspaceManager.ts")
        fs.copy("", "")
        // console.log(path.basename('C:\\temp\\myfile.html'));
    }
    constructor() {
    }

}

var instance: WorkspaceManager;

export function get(): WorkspaceManager {
    if (!instance) {
        instance = new WorkspaceManager();
    }
    return instance;
}