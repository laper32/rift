rift.metadata.add("1", "Value");
console.log("from metadata.ts=>111");

rift.tasks.impl("build", () => {
    console.log("build impl")
});