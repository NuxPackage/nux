use clap::ArgMatches;
use console::style;
use duct::cmd;
use indicatif::ProgressBar;

use crate::commands::util::doesTermSupportColor;
use std::str;
pub fn install_subcommand(app: ArgMatches) {
    let spinner = ProgressBar::new_spinner();
    let package_name = app.value_of("packagename").unwrap();
    if doesTermSupportColor() {
        println!(
            "{} {}",
            style("Installing package").green().bright(),
            style(package_name).bold()
        );
    } else {
        println!("Installing package {}", package_name)
    }

    let mut nix_install_cmds = cmd!("nix-env", "-i", package_name)
        .unchecked()
        .stdout_capture()
        .stderr_capture();
    if app.is_present("attr") {
        nix_install_cmds = cmd!("nix-env", "-iA", package_name)
            .unchecked()
            .stdout_capture()
            .stderr_capture();
        eprintln!("Using atttr")
    }
    if app.occurrences_of("unstable") == 1 {
        nix_install_cmds = cmd!("nix-env", "-f", "<nixos-unstable>", "-i", package_name)
            .unchecked()
            .stdout_capture()
            .stderr_capture();
        if app.is_present("attr") {
            nix_install_cmds = cmd!("nix-env", "-f", "<nixos-unstable>", "-iA", package_name)
                .unchecked()
                .stdout_capture()
                .stderr_capture();
        }
        if doesTermSupportColor() {
            println!(
                "{} {}",
                style("WARNING installing unstable package")
                    .yellow()
                    .bright(),
                style(package_name).bold()
            );
        } else {
            println!(
                "WARNING installing unstable package package {}",
                package_name
            )
        }
    }
    spinner.enable_steady_tick(20);

    let nix_install_cmd = nix_install_cmds.start().unwrap();

    let output = str::from_utf8(&nix_install_cmd.wait().unwrap().stderr).unwrap();

    spinner.finish_and_clear();

    if app.is_present("attr") {
        println!("Using atttr")
    }
    if output.contains("error") {
        if doesTermSupportColor() {
            println!(
                "{} {} {}",
                style("Package").red().bright(),
                style(package_name).bold(),
                style("had an error installing").red().bright()
            );
        } else {
            println!("Packcage {} had an error installing", package_name)
        }
        println!("Cause:");
        println!("{}", output);
    } else {
        if doesTermSupportColor() {
            println!(
                "{} {} {}",
                style("Package").green().bright(),
                style(package_name).bold(),
                style("was succesfully installed").green().bright()
            );
        } else {
            println!("Pakcage {} was succesfully installed", package_name)
        }
        if app.occurrences_of("debug") == 1 {
            println!("cause:");
            println!("{}", output);
        }
    }
}
