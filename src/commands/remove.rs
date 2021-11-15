use crate::commands::util::doesTermSupportColor;
use console::style;
use dialoguer::Confirm;
use duct::cmd;
use std::process::exit;
pub fn remove_command(app: clap::ArgMatches) {
    let spinner = indicatif::ProgressBar::new_spinner();
    let package_name = app.value_of("drvname").unwrap();

    if Confirm::new()
        .with_prompt(String::from("Are you sure you want to remove ") + package_name)
        .interact()
        .unwrap()
    {
    } else {
        if doesTermSupportColor() {
            println!("{}", style("Aborting removal...").red().bright().bold())
        } else {
            println!("Aborting removal...");
            exit(1);
        }
    }
    let mut nix_install_cmds = cmd!("nix-env", "-e", package_name)
        .unchecked()
        .stdout_capture()
        .stderr_capture();

    spinner.enable_steady_tick(20);

    let nix_install_cmd = nix_install_cmds.start().unwrap();

    let output = std::str::from_utf8(&nix_install_cmd.wait().unwrap().stderr).unwrap();
    spinner.finish_and_clear();

    if output.contains("error") {
        if doesTermSupportColor() {
            println!(
                "{} {} {}",
                style("Package").red().bright(),
                style(package_name).bold(),
                style("had an error removing").red().bright()
            );
        } else {
            println!("Packcage {} had an error removing", package_name)
        }
        println!("Cause:");
        println!("{}", output);
    } else {
        if doesTermSupportColor() {
            println!(
                "{} {} {}",
                style("Package").green().bright(),
                style(package_name).bold(),
                style("was succesfully removed").green().bright()
            );
        } else {
            println!("Pakcage {} was succesfully removed", package_name)
        }
        if app.occurrences_of("debug") == 1 {
            println!("cause:");
            println!("{}", output);
        }
    }
}
