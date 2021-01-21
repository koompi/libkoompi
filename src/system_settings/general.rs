use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{prelude::*, Error, Result};
use toml::{from_str, to_string_pretty};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ThemeManager {
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
fn reader(name: &str) -> Result<String> {
    let file = File::open(name);
    let mut data = String::new();

    match file {
        Ok(mut f) => {
            let rr = f.read_to_string(&mut data);
            match rr {
                Ok(_) => Ok(data),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

fn writer(name: &str, data: &Theme) -> Result<()> {
    let mut file = File::create(name)?;
    match file.write_all(to_string_pretty(data).unwrap().as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            name: String::from("KOOMPI"),
            desc: String::from("Theme for KOOMPI OS"),
        }
    }
}
