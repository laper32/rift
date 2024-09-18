// 考虑到V8的实际情况，我们需要提前启动runtime才能进行后面的操作。
// 但这样就代表着整个Engine都得全Async.

fn main() {
    engine::main()
}
