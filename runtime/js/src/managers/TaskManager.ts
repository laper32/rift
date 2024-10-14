class TaskManager {

}

var instance: TaskManager;

export function get(): TaskManager {
    if (!instance) {
        instance = new TaskManager();
    }
    return instance;
}