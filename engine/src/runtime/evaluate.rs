use crate::util::errors::RiftResult;

use super::loader::RiftModuleLoader;

pub async fn evaluate() -> RiftResult<()> {
    evaluate_impl().await
}

async fn evaluate_impl() -> RiftResult<()> {
    let (result_tx, result_rx) = tokio::sync::oneshot::channel::<RiftResult<()>>();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();

        let runtime = match runtime {
            Ok(runtime) => runtime,
            Err(error) => {
                let _ = result_tx.send(Err(error.into()));
                return;
            }
        };

        // Spawn the main JS task on the new runtime. See this issue for
        // more context on why this is required:
        // https://github.com/brioche-dev/brioche/pull/105#issuecomment-2241289605
        let result = runtime.block_on(async move {
            let module_loader = RiftModuleLoader::new();
        });
    });

    Ok(())
}
