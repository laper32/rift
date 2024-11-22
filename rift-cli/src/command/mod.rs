mod cli;

use cli::cli;

use crate::{de::ClapArgAction, engine, errors::RiftResult};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserCommand {
    name: String,
    about: Option<String>,
    before_help: Option<String>,
    after_help: Option<String>,
    subcommands: Option<Vec<UserCommand>>,
    package_name: String,
    args: Option<Vec<UserCommandArg>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserCommandArg {
    name: String,
    short: Option<char>,
    description: Option<String>,
    default: Option<serde_json::Value>,
    conflict_with: Vec<String>,
    heading: Option<String>,
    action: Option<ClapArgAction>,
}

#[derive(PartialEq)]
enum CommandManagerStatus {
    Unknown,
    Init,
    Ready,
}

pub struct CommandManager {
    status: CommandManagerStatus,
    commands: Vec<UserCommand>,
}

impl CommandManager {
    fn new() -> Self {
        Self {
            status: CommandManagerStatus::Unknown,
            commands: Vec::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<CommandManager> =
            once_cell::sync::Lazy::new(|| CommandManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn get_user_commands(&mut self) {
        if self.status == CommandManagerStatus::Ready {
            return;
        }

        self.status = CommandManagerStatus::Init;
        let result = serde_json::from_str::<Vec<UserCommand>>(&engine::get_user_commands());
        match result {
            Ok(commands) => {
                self.commands = commands;
                self.status = CommandManagerStatus::Ready;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    pub fn find_command_mut(&mut self, name: &str) -> Option<&mut UserCommand> {
        if self.status != CommandManagerStatus::Ready {
            return None;
        }
        self.commands.iter_mut().find(|c| c.name == name)
    }

    pub fn find_command(&self, name: &str) -> Option<&UserCommand> {
        if self.status != CommandManagerStatus::Ready {
            return None;
        }
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn remove_command(&mut self, name: &str) {
        if self.status != CommandManagerStatus::Ready {
            return;
        }
        self.commands.retain(|c| c.name != name);
    }

    pub fn has_command(&self, name: &str) -> bool {
        if self.status != CommandManagerStatus::Ready {
            return false;
        }
        self.commands.iter().any(|c| c.name == name)
    }

    pub fn to_clap_commands(&self) -> Option<Vec<clap::Command>> {
        if self.status != CommandManagerStatus::Ready {
            return None;
        }
        let mut ret = Vec::new();

        fn make_clap_command(user_command: &UserCommand) -> clap::Command {
            let mut cmd = clap::Command::new(user_command.name.clone());
            if let Some(about) = user_command.about.clone() {
                cmd = cmd.about(about);
            }
            if let Some(before_help) = user_command.before_help.clone() {
                cmd = cmd.before_help(before_help);
            }
            if let Some(after_help) = user_command.after_help.clone() {
                cmd = cmd.after_help(after_help);
            }
            if user_command.args.is_some() {
                let mut args: Vec<clap::Arg> = Vec::new();
                user_command.args.as_ref().unwrap().iter().for_each(|arg| {
                    let mut arg_actual = clap::Arg::new(arg.name.clone()).long(arg.name.clone());
                    if let Some(short) = arg.short {
                        arg_actual = arg_actual.short(short);
                    }

                    if let Some(description) = arg.description.clone() {
                        arg_actual = arg_actual.help(description);
                    }

                    if arg.conflict_with.len() > 0 {
                        arg_actual = arg_actual.conflicts_with_all(&arg.conflict_with);
                    }

                    if let Some(heading) = arg.heading.clone() {
                        arg_actual = arg_actual.help_heading(heading);
                    }

                    if let Some(action) = arg.action.clone() {
                        let action_into: clap::ArgAction = action.into();
                        arg_actual = arg_actual.action(action_into);
                    }

                    if let Some(default) = arg.default.clone() {
                        arg_actual = arg_actual.default_value(default.to_string());
                    }

                    args.push(arg_actual);
                });
                cmd = cmd.args(args);
            }

            if user_command.subcommands.is_some() {
                let subcommands = user_command.subcommands.as_ref().unwrap();
                for subcommand in subcommands {
                    let subcmd = make_clap_command(subcommand);
                    cmd = cmd.subcommand(subcmd);
                }
            }

            return cmd;
        }

        self.commands.iter().for_each(|user_command| {
            let cmd = make_clap_command(user_command);
            ret.push(cmd);
        });

        return Some(ret);
    }

    pub fn exec_command(&self) -> RiftResult<()> {
        let matches = cli().get_matches();
        let (cmd, subcommand_args) = match matches.subcommand() {
            Some((name, args)) => (name, args),
            _ => {
                cli().print_help()?;
                return Ok(());
            }
        };
        return Ok(());
    }

    pub fn dump_user_commands(&self) {
        println!("dump_user_commands");
        println!("{}", serde_json::to_string_pretty(&self.commands).unwrap());
    }
}

//     pub fn exec_command(&self) -> RiftResult<()> {
//         let matches = cli::cli().get_matches();
//         let (cmd, subcommand_args) = match matches.subcommand() {
//             Some((name, args)) => (name, args),
//             _ => {
//                 cli::cli().print_help()?;
//                 return Ok(());
//             }
//         };
//         println!("command: {cmd}");
//         println!("args:");
//         CommandManager::instance()
//             .find_command(cmd)
//             .unwrap()
//             .args
//             .iter()
//             .for_each(|arg| {
//                 let arg = subcommand_args.get_one::<String>(&arg.name);
//                 println!("{:?}", arg);
//             });
//         return Ok(());
//     }
// }
