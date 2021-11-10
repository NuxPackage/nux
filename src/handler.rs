pub mod cli_handler {
    use crate::commands::install;
    use crate::commands::upgrade;

    pub fn handle_cli(matcher: clap::ArgMatches) {
        if let Some(ref subcmd) = matcher.subcommand_matches("install") {
            install::install_subcommand(subcmd.to_owned().to_owned());
        } else if let Some(ref subcmd) = matcher.subcommand_matches("upgrade") {
            upgrade::upgrade_subcommand(subcmd.to_owned().to_owned());
        }
    }
}
