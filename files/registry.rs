//! Windows registry read helpers for U He Zebra 3.

pub const APP_KEY: &str = "Software\\U\\U He Zebra 3";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\U He Zebra 3";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\U He Zebra 3") },
        RegValue { name: "Version",     value: String::from("v4.2.23") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("U He Zebra 3") },
        RegValue { name: "Publisher",       value: String::from("U") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\U He Zebra 3\\uninstall.exe\"") },
    ]
}
