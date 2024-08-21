use std::{ffi::OsString, process::exit, vec};

use anyhow::anyhow;
use clap::{ArgMatches, Command};
use engine::{
    task::{alias::AliasManager, TaskManager},
    util::{
        errors::{
            cli::{CliError, CliResult},
            RiftResult,
        },
        get_cargo_project_root,
    },
};

fn main() {
    let result = cli_main();
    match result {
        Ok(_) => {}
        Err(_e) => {}
    }
}

fn cli_main() -> CliResult {
    let matches = cli().try_get_matches();
    match matches {
        Ok(matches) => Ok(()),
        Err(e) => {
            println!("Error: {:?}", e.print());
            exit(e.exit_code())
        }
    }
    // let matches = cli().try_get_matches()?;

    // let expanded_args = expand_aliases(matches, vec![])?;
}

pub fn cli() -> Command {
    TaskManager::instance().add_possible_tasks(
        &get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("07_task")
            .join("Task.toml"),
    );
    TaskManager::instance().add_task("build".to_owned(), || {
        println!("TaskManager::Instance::AddTask::Invocation (Build)");
    });
    AliasManager::instance().load_alias(
        get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("09_alias")
            .join("Alias.toml"),
    );

    Command::new("Rift")
        .version("1.0") // TODO: Replace with actual project version
        .about("Rift build system")
        .subcommands(TaskManager::instance().to_commands())
}

// 我们得考虑到 rift run 后面接别的二进制文件的情况。
fn expand_aliases(
    args: ArgMatches,
    mut already_expanded: Vec<String>,
) -> Result<ArgMatches, CliError> {
    println!("ExpandAliases in");
    // 我不知道cargo是怎么展开的
    // 直接.subcommand直接就None了..
    println!("{:?}", args);
    match args.subcommand() {
        Some((cmd, sub_args)) => {
            println!("Some => in => args: {args:?}");
            Ok(args)
        }
        None => {
            println!("args.subcommand() out");
            println!("{:?}", args);
            // let has_alias =
            //     AliasManager::instance().has_alias(&String::from(args.into()));
            // println!("HasAlias: {has_alias}");
            Ok(args)
        }
    }
}

// fn expand_aliases(args: ArgMatches, mut already_expanded: Vec<String>) -> RiftResult<ArgMatches> {
//     println!("ExpandAliases in");
//     if let Some((cmd, sub_args)) = args.subcommand() {
//         println!("args.subcommand() in");
//         let aliased_cmd = AliasManager::instance().to_command_vec(cmd);
//         println!("{:?}", aliased_cmd);
//         match aliased_cmd {
//             Ok(Some(alias)) => {
//                 let mut alias = alias
//                     .into_iter()
//                     .map(|s| OsString::from(s))
//                     .collect::<Vec<_>>();
//                 alias.extend(
//                     sub_args
//                         .get_many::<OsString>("")
//                         .unwrap_or_default()
//                         .cloned(),
//                 );
//                 println!("{:?}", alias);

//                 let new_args = cli().try_get_matches_from(alias)?;
//                 let Some(new_cmd) = new_args.subcommand_name() else {
//                     return Err(anyhow!(
//                         "subcommand is required, add a subcommand to the command alias `alias.{cmd}`"
//                     )
//                         .into());
//                 };
//                 already_expanded.push(cmd.to_string());
//                 if already_expanded.contains(&new_cmd.to_string()) {
//                     return Err(anyhow!(
//                         "alias {} has unresolvable recursive definition: {} -> {}",
//                         already_expanded[0],
//                         already_expanded.join(" -> "),
//                         new_cmd,
//                     )
//                     .into());
//                 }
//                 let expanded_args = expand_aliases(new_args, already_expanded)?;
//                 return Ok(expanded_args);
//             }
//             Ok(None) => {
//                 println!("Ok(None)");
//             }
//             Err(e) => {
//                 println!("Err(e)");
//                 return Err(e.into());
//             }
//         };
//     } else {
//         println!("args.subcommand() out");
//     };
//     Ok(args)
// }
