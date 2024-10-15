export namespace bootstrap {
    function init(): Boolean {
        console.log('Runtime bootstrap')
        return true;
    }

    function shutdown() {
        console.log('Runtime shutdown')
    }
}