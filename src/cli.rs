pub mod app {
    pub fn make_clap_app() -> clap::App<'static, 'static> {
        return clap::App::new("nux")
        .version("0.1.0")
        .author("Jakob Neufeld <jakob@mast3rsoft.com>")
        .about("Nux is a simple wrapper for the nix cli tools. This tool is made for NixOS but can be run somewhere else.")
        .subcommand(
            clap::SubCommand::with_name("install")
                .about("Command for installing a package")
                .alias("inst")
                .after_help(crate::help::INSTALL_DISCUSSION)
                .arg(clap::Arg::with_name("attr")
                     .short("a")
                     .long("attribute")
                     .help("Flag for setting if the package selector contains a package attribute, e.g., instead of xmonad, you would use haskellPackages.xmonad")
                )
                .arg(clap::Arg::with_name("packagename")
                     .help("The package selector (name). e.g. nux install firefox.")
                     .required(true)
                )
                .arg(clap::Arg::with_name("unstable")
                     .help("Installs the unstable version of the package. Only works if unstable channel was added. Does not work with attribute because the attribute is set by default.")
                     .long("unstable")
                     .short("u")
                     )
                .arg(clap::Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Shows the output of the nix cmds. For debugging purposes.")
                )

        )
             .subcommand(clap::App::new("upgrade")
                            .arg(clap::Arg::with_name("packagename")
                                 .help("The target package to upgrade. This is optional")
                            )
                )

        .setting(clap::AppSettings::SubcommandRequiredElseHelp);
    }
}
