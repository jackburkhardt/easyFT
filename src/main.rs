use winreg::{enums::HKEY_CURRENT_USER, RegKey};
use std::{fmt::Result, io};

const APPLICATION_NAME: &str = "easyFT";
const APPLICATION_VERSION: &str = "0.1";

fn main() {
    println!("Hello, world!");

    let res = update_registry();
    match res {
        Ok(_) => println!("Registry updated successfully"),
        Err(e) => println!("Error updating registry: {}", e),
    }

}

fn update_registry() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let exe_path = format!("C:\\Users\\{}\\AppData\\Local\\Programs\\{}\\{}.exe", std::env::var("USERNAME").unwrap(), APPLICATION_NAME, APPLICATION_NAME);

    // File context menus
    let (key, _disp) = hkcu.create_subkey(format!("Software\\Classes\\*\\shell\\{APPLICATION_NAME}"))?;
    key.set_value("", &"Send with easyFT")?;
    key.set_value("Icon", &exe_path)?;

    let (key, _disp) = hkcu.create_subkey(format!("Software\\Classes\\*\\shell\\{APPLICATION_NAME}\\command"))?;
    key.set_value("", &format!("{exe_path} %1"))?;

    // Directory context menus
    let (key, _disp) = hkcu.create_subkey(format!("Software\\Classes\\Directory\\shell\\{APPLICATION_NAME}"))?;
    key.set_value("", &"Send folder with easyFT")?;
    key.set_value("Icon", &exe_path)?;

    let (key, _disp) = hkcu.create_subkey(format!("Software\\Classes\\Directory\\shell\\{APPLICATION_NAME}\\command"))?;
    key.set_value("", &format!("{exe_path} %1"))?;

    Ok(())
}
