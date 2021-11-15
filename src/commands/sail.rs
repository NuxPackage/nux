use serde_json::from_str;
use serde_json::Value;
use clap::ArgMatches;
use console::style;
use duct::cmd;
use crate::commands::util::doesTermSupportColor;
pub fn sail_command(app: ArgMatches ) {


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
    let query_dict = nixpkgs_result_search.as_object().expect("Invalid Nix Json!");
    if query_dict.is_empty() {
        if doesTermSupportColor() {
            println!("{}", style("Yarr me boat ye no find ye package in the sea. Ye try rewriting your query!").bright().bold().red())
        } else {
            println!("Yarr me boat ye no find ye package in the sea. Ye try rewriting your query!")
        }
        std::process::exit(1);
    }


}
