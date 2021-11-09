use duct::cmd;
use serde_json::Value;

pub fn get_package_version(packagename: String) -> Option<String> {
    let pacakgeid = String::from("nixos.") + &*packagename;
    let mut nix_install_cmds = cmd!("nix-env", "-qa", pacakgeid.clone())
        .unchecked()
        .stdout_capture()
        .stderr_capture();

    let mut nix_install_cmd = nix_install_cmds.start().unwrap();
    let output = std::str::from_utf8(&nix_install_cmd.wait().unwrap().stderr).unwrap();
    let output_json: Value = serde_json::from_str(output).unwrap();
    if let Some(pakcage_object) = output_json.get(&pacakgeid.clone()) {
        return Some(
            pakcage_object
                .get("version")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
        );
    } else {
        // eprint!("Sorry, the package could not be found. Please look at search.nixos.org");
        return None;
    }
    return Some(String::from("ddd"));
}
