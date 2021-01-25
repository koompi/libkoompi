// This code was autogenerated with `dbus-codegen-rust -s -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerSecretAgent {
    fn get_secrets(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, connection_path: dbus::Path, setting_name: &str, hints: Vec<&str>, flags: u32) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error>;
    fn cancel_get_secrets(&self, connection_path: dbus::Path, setting_name: &str) -> Result<(), dbus::Error>;
    fn save_secrets(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, connection_path: dbus::Path) -> Result<(), dbus::Error>;
    fn delete_secrets(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, connection_path: dbus::Path) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNetworkManagerSecretAgent for blocking::Proxy<'a, C> {

    fn get_secrets(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, connection_path: dbus::Path, setting_name: &str, hints: Vec<&str>, flags: u32) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.SecretAgent", "GetSecrets", (connection, connection_path, setting_name, hints, flags, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::PropMap>, )| Ok(r.0, ))
    }

    fn cancel_get_secrets(&self, connection_path: dbus::Path, setting_name: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.SecretAgent", "CancelGetSecrets", (connection_path, setting_name, ))
    }

    fn save_secrets(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, connection_path: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.SecretAgent", "SaveSecrets", (connection, connection_path, ))
    }

    fn delete_secrets(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>, connection_path: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.SecretAgent", "DeleteSecrets", (connection, connection_path, ))
    }
}