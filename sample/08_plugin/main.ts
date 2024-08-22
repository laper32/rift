namespace rift {
    export enum EventIdentity {
        Load,
        Unload,
        AllLoad
    } // enum 遇到自定义事件直接炸


    export function RegisterEvent(event: EventIdentity, callback: Function) {
        // rift.RegisterEvent(event, callback)
    }
}
async function main() {
    rift.RegisterEvent(rift.EventIdentity.Load, async () => {
    })
}