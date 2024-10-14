class PluginManager {

}

var instance: PluginManager

export function get(): PluginManager {
    if (!instance) {
        instance = new PluginManager();
    }
    return instance;
}