use crate::commands::util::doesTermSupportColor;
use clap::ArgMatches;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use duct::cmd;
use serde_json::from_str;
use serde_json::Value;

pub fn sail_command(app: ArgMatches) {
    if doesTermSupportColor() {
        println!("{}", style("⛵⛵⛵Yarr welcome to me nixpkgs boat. Me will find ye package. Please wait for a while so me can sail the Sea of Nix⛵⛵⛵").bright().bold().green())
    } else {
        println!("Yarr welcome to me nixpkgs boat. Me will find ye package. Please wait for a while so me can sail the Sea of Nix")
    }

    let mut query = app.value_of("query").unwrap();
    let mut nix_install_cmds = cmd!("nix", "search", "nixpkgs", query, "--json")
        .unchecked()
        .stdout_capture()
        .stderr_capture();

    let nix_install_cmd = nix_install_cmds.start().unwrap();

    let output = std::str::from_utf8(&nix_install_cmd.wait().unwrap().stdout).unwrap();
    let mut nixpkgs_result_search: Value = from_str(output).unwrap();
    let query_dict = nixpkgs_result_search
        .as_object()
        .expect("Invalid Nix Json!");
    if query_dict.is_empty() {
        if doesTermSupportColor() {
            println!(
                "{}",
                style(
                    "Yarr me boat ye no find ye package in the sea. Ye try rewriting your query!"
                )
                .bright()
                .bold()
                .red()
            )
        } else {
            println!("Yarr me boat ye no find ye package in the sea. Ye try rewriting your query!")
        }
        std::process::exit(1);
    }
    if doesTermSupportColor() {
        println!(
            "{}",
            style("Yarr me have some packages related to ye query!")
                .bright()
                .bold()
                .green()
        )
    } else {
        println!("Yarr me have some packages related to ye query")
    }

    // Now we print all the packages!

    let mut found_packages = Vec::new();

    for found_package_key in query_dict.keys() {
        found_packages.push(found_package_key);
    }
    let mut found_packages_non_reversed = found_packages.clone();
    found_packages.reverse();
    let mut i = found_packages.len();
    for found_package in found_packages {
        let mut package_object = query_dict.get(found_package).unwrap().as_object().unwrap();
        let package_name = package_object.get("pname").unwrap().as_str().unwrap();
        let version = package_object.get("version").unwrap().as_str().unwrap();
        let description = package_object.get("description").unwrap().as_str().unwrap();
        println!(
            "* {} {} ({})",
            style(i).bold(),
            style(package_name).bold().green().underlined(),
            style(version).bold().italic()
        );
        println!("    {}", description);

        i -= 1;
    }
    let mut max = found_packages_non_reversed.to_owned().clone().len();
    let input: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What package you fancy")
        .validate_with(move |a: &usize| -> Result<(), &'static str> { length_validator(*a, max) })
        .interact_text()
        .unwrap();
    let chosen_pkg_key = found_packages_non_reversed[input - 1];

    let mut chosen_package_object = query_dict.get(chosen_pkg_key).unwrap().as_object().unwrap();

    let package_name = chosen_package_object
        .get("pname")
        .unwrap()
        .as_str()
        .unwrap();
    let mut nix_install_cmds = cmd!("nix-env", "-i", package_name)
        .unchecked()
        .stdout_capture()
        .stderr_capture();

    let nix_install_cmd = nix_install_cmds.start().unwrap();

    let output = std::str::from_utf8(&nix_install_cmd.wait().unwrap().stderr).unwrap();

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

fn length_validator(chosen_option: usize, package_length: usize) -> Result<(), &'static str> {
    if chosen_option > package_length {
        return Err("Sorry, ye can only pick a package id from what I have!");
    }
    return Ok(());
}
