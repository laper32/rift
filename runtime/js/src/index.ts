import * as workspaceManager from "./managers/WorkspaceManager.ts";

export namespace bootstrap {
    export function init() {
        console.log('bootstrap init');
        workspaceManager.get().call();
        workspaceManager.get().call();
        // WorkspaceManager.instance.call();
        return true;
    }
    export function shutdown() {
        console.log('bootstrap shutdown');
    }
}
