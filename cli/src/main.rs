// deno初始化以后整个jsRuntime就已经在后台挂机了
// 且经过测试得知，native传的函数指针是可以保存+在整个program的其他地方去call的。
// 基于这个发现，换句话说我们其实可以不用担心async传染的问题。

use engine::{task::TaskManager, workspace::WorkspaceManager};

fn setup_panic_hook() {
    // This function does two things inside of the panic hook:
    // - Tokio does not exit the process when a task panics, so we define a custom
    //   panic hook to implement this behaviour.
    // - We print a message to stderr to indicate that this is a bug in Deno, and
    //   should be reported to us.
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        eprintln!("\n============================================================");
        eprintln!("Detected panic in Rift.");
        eprintln!("Report this at https://github.com/laper32/rift/issues/new.");
        eprintln!("If you can reliably reproduce this panic, include the");
        eprintln!("reproduction steps and re-run with the RUST_BACKTRACE=1 env");
        eprintln!("var set and include the backtrace in your report.");
        eprintln!();
        eprintln!(
            "Platform: {} {}",
            std::env::consts::OS,
            std::env::consts::ARCH
        );
        // todo: print rift version
        eprintln!("Args: {:?}", std::env::args().collect::<Vec<_>>());
        orig_hook(panic_info);
        std::process::exit(1);
    }));
}

fn main() {
    setup_panic_hook();
    let cwd = std::env::current_dir().unwrap();
    WorkspaceManager::instance().set_current_manifest(&cwd.join("Rift.toml"));
    match WorkspaceManager::instance().load_packages() {
        Ok(_) => {
            /* TaskManager::instance().register_tasks();
            TaskManager::instance().run_tasks(); */
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(-1)
        }
    }

    // engine::init();

    // let command = clap::Command::new("rift").subcommands(TaskManager::instance().to_commands());
    // let matches = command.get_matches();

    engine::shutdown();
}
