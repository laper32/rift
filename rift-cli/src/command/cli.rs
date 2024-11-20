use super::CommandManager;

pub(crate) fn cli() -> clap::Command {
    clap::Command::new("rift")
        .about(r#"Rift, a multi-language, cross platform build system."#)
        .version("0.1.0")
        .subcommands(CommandManager::instance().to_commands().unwrap())
}
