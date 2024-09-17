rift.plugins.onLoad.addListener(() => {
    console.log("Loading plugin");
});

rift.plugins.onUnload.addListener(() => {
    console.log("Unloading plugin");
});

rift.plugins.onAllLoaded.addListener(() => {
    console.log("All plugins loaded");
});