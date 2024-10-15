import * as workspaceManager from "./managers/WorkspaceManager.ts";
import * as version from "./engine/version.ts";

export namespace bootstrap {
    export function init() {
        console.log('bootstrap init');
        console.log(version.get().gitHash);

        workspaceManager.get().call();
        workspaceManager.get().call();
        return true;
    }
    export function shutdown() {
        console.log('bootstrap shutdown');
    }
}

