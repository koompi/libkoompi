use lazy_static::lazy_static;
use std::path::PathBuf;

// Resource & Config Locations
lazy_static! {
    pub static ref DATA_DIRS: Vec<PathBuf> = std::env::var("XDG_DATA_DIRS").unwrap_or(String::from("/usr/local/share:/usr/share")).split(':').filter_map(|dir| {
        let path = PathBuf::from(dir);
        if path.exists() && path.is_dir() {
            Some(path)
        } else {
            None
        }
    }).collect();
    pub static ref LOCAL_DATA: PathBuf = dirs_next::data_dir().unwrap();

    pub static ref CONF_DIRS: Vec<PathBuf> = std::env::var("XDG_CONFIG_DIRS").unwrap_or(String::from("/etc/xdg")).split(':').filter_map(|dir| {
        let path = PathBuf::from(dir);
        if path.exists() && path.is_dir() {
            Some(path)
        } else {
            None
        }
    }).collect();
    pub static ref LOCAL_CONF: PathBuf = dirs_next::config_dir().unwrap();
}

pub const PKEXEC: &str = "pkexec";