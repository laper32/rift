use anyhow::Error;

// async有传染性，我们需要在后台跑js runtime，这玩意得async
// 可能有让它sync的方法？或许是用message机制?
#[tokio::main]
async fn main() -> Result<(), Error> {
    engine::init();

    engine::shutdown();

    Ok(())
}
