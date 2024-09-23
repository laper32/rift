// deno初始化以后整个jsRuntime就已经在后台挂机了
// 且经过测试得知，native传的函数指针是可以保存+在整个program的其他地方去call的。
// 基于这个发现，换句话说我们其实可以不用担心async传染的问题。

use std::env;

use engine::{
    manifest::MANIFEST_IDENTIFIER, plsys::PluginManager, runtime, task::TaskManager,
    workspace::WorkspaceManager,
};

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
        eprintln!("Rift version: {}", env!("CARGO_PKG_VERSION"));
        eprintln!("Args: {:?}", std::env::args().collect::<Vec<_>>());
        orig_hook(panic_info);
        std::process::exit(1);
    }));
}

fn main() {
    setup_panic_hook();
    let args = std::env::args().collect::<Vec<_>>();
    let mut cmd = clap::Command::new("rift")
        .about("Rift, a muitlple-language build system.")
        .version(env!("CARGO_PKG_VERSION"))
        .after_help("All subcommands will be traded as 'Task', they will be provided when you are in a specific project.");

    if args.len() == 1 {
        cmd.print_help().unwrap();
    } else {
        let cwd = std::env::current_dir().unwrap();
        WorkspaceManager::instance().set_current_manifest(&cwd.join(MANIFEST_IDENTIFIER));
        match WorkspaceManager::instance().load_packages() {
            Ok(_) => {
                runtime::init();
                PluginManager::instance().evaluate_entries();
                PluginManager::instance().activate_instances();
                let cmd = cmd.subcommands(TaskManager::instance().to_commands());
                match cmd.try_get_matches() {
                    Ok(cmd) => match cmd.subcommand() {
                        Some((name, _)) => {
                            let task = TaskManager::instance().get_task(name);
                            if task.is_none() {
                                eprintln!("Task '{}' not found.", name);
                                return;
                            }
                            let task = task.unwrap();
                            task.get_fn().unwrap().invoke();
                        }
                        None => {}
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
                // match result = cmd.get_matches().subcommand() {
                //     Some(_) => todo!(),
                //     None => todo!(),
                // }

                // println!("{:?}", classified_commands.len());
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }

    PluginManager::instance().deactivate_instances();
}

/*
    #[test]
    fn test_load_plugins() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("02_single_target_with_project")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        match WorkspaceManager::instance().load_packages() {
            Ok(_) => {
                init();
                PluginManager::instance().evaluate_entries();
                PluginManager::instance().activate_instances();
                PluginManager::instance().deactivate_instances();
                // PluginManager::instance().register_plugin_listeners();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
*/
