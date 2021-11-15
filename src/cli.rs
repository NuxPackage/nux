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
             .subcommand(clap::SubCommand::with_name("upgrade")
                            .arg(clap::Arg::with_name("packagename")
                                 .help("The target package to upgrade. This is optional")
                            )
             )
          .subcommand(clap::SubCommand::with_name("remove")
               .about("Removes a package from to system")
               .arg(clap::Arg::with_name("drvname")
               .help("The target derivation is getting uninstalled.")
               .required(true)

               )
                      )
                      .subcommand(clap::SubCommand::with_name("sail")
                                  .about("Interactively discover packages and install them!")
                                  .arg(clap::Arg::with_name("query")
                                       .required(true)
                                       .validator(sail_validator)
                                       .help("Yarr, ye sail around nixpkgs! Ye shall try it out!")
                                  )
     )

        .setting(clap::AppSettings::SubcommandRequiredElseHelp);
    }
    fn sail_validator(arg: String) -> Result<(), String> {
        if (arg.len() >= 3) {
            Ok(())
        } else {
            Err(String::from(
                "Sorry, minimum 3 three characters in search query!",
            ))
        }
    }
}
