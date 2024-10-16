import * as path from "@std/path";
import * as toml from  "@std/toml";
import * as fs from "@std/fs";
const Deno = globalThis.Deno;
class WorkspaceManager {
    public call() {
        let p = 'D:/workshop/projects/rift/runtime/js/src/managers/WorkspaceManager.ts';
        let result = path.isAbsolute(p)
        let text = Deno.readTextFile(p);
        // console.log(text)
        
        console.log(p,"is absolute path:", result)
    }    
}

var instance: WorkspaceManager;

export function get(): WorkspaceManager {
    if (!instance) {
        instance = new WorkspaceManager();
    }
    return instance;
}