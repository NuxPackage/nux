use crate::commands::util;
use crate::commands::util::doesTermSupportColor;
use clap::ArgMatches;
use console::style;
use dialoguer::Confirm;
use duct::cmd;
pub fn upgrade_subcommand(app: clap::ArgMatches) {
    /////////////////////////////////////////////////////////////////////////////////////////
    // let mut nix_install_cmds = cmd!("nix-env", "-i", package_name)                      //
    //     .unchiecked()                                                                    //
    //     .stdout_capture()                                                               //
    //     .stderr_capture();                                                              //
    // let mut nix_install_cmd = nix_install_cmds.start().unwrap();                        //
    //                                                                                     //
    // let output = std::str::from_utf8(&nix_install_cmd.wait().unwrap().stderr).unwrap(); //
    /////////////////////////////////////////////////////////////////////////////////////////

    // TODO WORK ON THIS
    if app.is_present("packagename") {
        let package_name = app.value_of("packagename").unwrap();

        let mut nix_install_cmds = cmd!("nix-env", "-u", app.value_of("packagename").unwrap())
            .unchecked()
            .stdout_capture()
            .stderr_capture();
        let mut nix_install_cmd = nix_install_cmds.start().unwrap();

        let output = std::str::from_utf8(&nix_install_cmd.wait().unwrap().stderr).unwrap();
        if output.contains("error") {
            if doesTermSupportColor() {
                println!(
                    "{} {} {}",
                    style("Package").red().bright(),
                    style(package_name).bold(),
                    style("had an error updating").red().bright()
                );
            } else {
                println!("Packcage {} had an error updating", package_name)
            }
        } else {
            if doesTermSupportColor() {
                println!(
                    "{} {} {}",
                    style("Package").green().bold().bright(),
                    style(package_name).bold(),
                    style("has been succesfully installed")
                        .green()
                        .bold()
                        .bright()
                );
            } else {
                println!("Packcage {} has been succesfully installed", package_name)
            }
        }
    } else {
        if Confirm::new()
            .with_prompt("Do you want to update all packages")
            .interact()
            .unwrap()
        {
            _updateAllPackages()
        } else {
            if doesTermSupportColor() {
                println!("{}", style("Aborting update...").red().bright().bold())
            } else {
                println!("Aborting update...")
            }
        }
    }
}
fn _updateAllPackages() {}
