use crate::commands::util;
use crate::commands::util::doesTermSupportColor;
use clap::ArgMatches;
use console::style;
use duct::cmd;
use dialoguer::Confirm;
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
    if app.is_present("packagename") {















    } else {

        if Confirm::new().with_prompt("Do you want to update all packages").interact().unwrap() {
            _updateAllPackages()
        } else {
            if doesTermSupportColor() {

            }
        }
    }
}
fn _updateAllPackages() {

}
