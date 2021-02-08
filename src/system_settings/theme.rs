use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{prelude::*, Error, Result};
use toml::{from_str, to_string_pretty};
use std::fs;


const HOME: &'static str = env!("HOME");
fn main() {
    //  generate default
    let theme = Theme::default();
    create_dir();
    // write data to file
    let my_toml = to_string_pretty(&theme).unwrap();
    writer("theme.conf", &theme).unwrap();

    // read data from file
    let str_data = reader("theme.conf");
    let data: Theme = from_str(&str_data.unwrap()).unwrap();

    println!("{:#?}", data.info.name);
}

pub fn reader(name: &str) -> Result<String> {
    let path = std::path::Path::new(format!("{}/.config/koompi/theme", HOME).as_str()).join(name);

    std::fs::read_to_string(path)
    
}

pub fn writer(name: &str, data: &Theme) -> Result<()> {
    
    let path = std::path::Path::new(format!("{}/.config/koompi/theme", HOME).as_str()).join(name);
   
    let mut file = File::create(path).unwrap();
    match file.write_all(to_string_pretty(data).unwrap().as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
        }
    
}
pub fn create_dir() -> std::io::Result<()>{
    fs::create_dir_all(format!("{}/.config/koompi/theme",HOME))?;
    Ok(())
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Theme {
    info: Info,
    button: Button,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    name: String,
    desc: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Button {
    border: u8,
    backgroud: String,
    color: String,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            name: String::from("KOOMPI"),
            desc: String::from("Theme for KOOMPI OS"),
        }
    }
}
