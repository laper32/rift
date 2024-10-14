
class WorkspaceManager {
    private value: number = 0;
    public call() {
        this.value++;
        console.log('WorkspaceManager call =>', this.value);
    }
}

var instance: WorkspaceManager;

export function get(): WorkspaceManager {
    if (!instance) {
        instance = new WorkspaceManager();
    }
    return instance;
}