// deno初始化以后整个jsRuntime就已经在后台挂机了
// 且经过测试得知，native传的函数指针是可以保存+在整个program的其他地方去call的。
// 基于这个发现，换句话说我们其实可以不用担心async传染的问题。

#[tokio::main]
async fn main() {
    engine::main()
}
