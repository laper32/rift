
export namespace bootstrap {
    export function init() {
        console.log('bootstrap init');
        return true;
    }
    export function shutdown() {
        console.log('bootstrap shutdown');
    }
}
