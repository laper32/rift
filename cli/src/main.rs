use engine::errors::RiftResult;

#[tokio::main]
async fn main() -> RiftResult<()> {
    runtime::init().await
    // match runtime::init().await {
    //     Ok(_) => {
    //         let shutdown_result = runtime::shutdown();
    //         match shutdown_result {
    //             Ok(_) => {
    //                 std::process::exit(0);
    //             }
    //             Err(e) => {
    //                 eprintln!("Error: {:?}", e);
    //                 std::process::exit(1);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {:?}", e);
    //         std::process::exit(1);
    //     }
    // }
}
