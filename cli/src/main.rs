use clap::Command;
fn main() {
    let _matches = Command::new("Rift")
        .version("1.0") // TODO: Replace with actual project version
        .about("Rift build system")
        .get_matches();

    engine::init();

    engine::shutdown();
}

/*
    pub fn expand_aliases(&self, args: ArgMatches) -> RiftResult<ArgMatches> {
        if let Some((cmd, sub_args)) = args.subcommand() {
            let aliased_cmd = self.to_command_vec(cmd);
            match aliased_cmd {
                Ok(Some(alias)) => {
                    let mut alias = alias
                        .into_iter()
                        .map(|s| OsString::from(s))
                        .collect::<Vec<_>>();
                    alias.extend(
                        sub_args
                            .get_many::<OsString>("")
                            .unwrap_or_default()
                            .cloned(),
                    );

                    // let new_args =
                }
                Ok(None) => {}
                Err(e) => return Err(e.into()),
            }
        };
        // let sub_cmd = args.subcommand();
        // let aliased_cmd = self.to_command_vec(sub_cmd.i)
        Ok(args)
    }
*/
