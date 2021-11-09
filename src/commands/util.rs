use std::env::var;
pub fn doesTermSupportColor() -> bool {
    match var("NO_COLOR") {
        Ok(_String) => return false,
        _ => {}
    }
    if let Ok(str) = var("COLORTERM") {
        match str.as_str() {
            "24bit" => return true,
            "truecolor" => return true,
            _ => {}
        }
    }

    return false;
}
