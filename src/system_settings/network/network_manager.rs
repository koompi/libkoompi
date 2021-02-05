mod net_test {
    use dbus::arg::messageitem::Props;
    use dbus::blocking::Connection;
    use std::time::Duration;
    pub fn enable(is_enable: bool) -> Result<(), dbus::Error> {
        let conn = Connection::new_system()?;
        let proxy = conn.with_proxy(
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            Duration::from_millis(1000),
        );
        let (): () = proxy.method_call("org.freedesktop.NetworkManager", "Enable", (is_enable,))?;
        Ok(())
    }

    pub fn property(
        name: &str,
        path: &str,
        inter: &str,
        props: &str,
        timeout: i32,
    ) -> Result<dbus::arg::messageitem::MessageItem, dbus::Error> {
        let c = dbus::ffidisp::Connection::new_system()?;
        let p = Props::new(&c, name, path, inter, timeout);
        match p.get(props) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
    pub fn bluetooth_info() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::new_system()?;
        let proxy = conn.with_proxy(
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            Duration::from_millis(1000),
        );
        let (name,): (Vec<String>,) =
            proxy.method_call("org.freedesktop.NetworkManager", "GetAccessPoints", ())?;
        println!("BLUETOOTH: {:?}", name);
        Ok(())
    }
    pub fn get_all_device() -> Result<(), Box<dyn std::error::Error>> {
        // Start a connection to the system bus.
        let c = Connection::new_system()?;

        // Make a "ConnPath" struct that just contains a Connection, a destination and a path.
        let p = c.with_proxy(
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            Duration::from_millis(2000),
        );
        // use crate::system_settings::network::OrgFreedesktopNetworkManager;
        use crate::system_settings::network::OrgFreedesktopNetworkManager;
        // let metadata = p.get_all_devices()?;
        // println!("wireless status: {}", p.wireless_enabled()?);
        // p.set_wireless_enabled(true)?;
        let acesspoints = p.wireless_enabled()?;
        println!("{:?}", acesspoints);
        // for path in metadata {
        //     println!("path : {:?}", path);
        // }
        Ok(())
    }
}
use std::io::Read;
use std::process::{Command, Stdio};
#[derive(Default, Debug)]
pub struct NetworkManager<'l> {
    ssid: &'l str,
    channel: &'l str,
    freq: &'l str,
    rate: &'l str,
    signal: u8,
    security: &'l str,
}

impl<'l> NetworkManager<'l> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn get_access_points(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut vec: Vec<String> = Vec::new();
        // let mut normal_bytes = String::new();
        let output = Command::new("nmcli")
            .args(&["-f", "SSID", "device", "wifi", "list"])
            .output();
        match output {
            Ok(data) => match String::from_utf8(data.stdout) {
                Ok(new_data) => {
                    new_data.lines().map(ToString::to_string).for_each(|d| {
                        if d != "--" || d != "SSID" {
                            vec.push(d);
                        } else {
                            {}
                        }
                    });
                    vec.sort();
                    Ok(vec)
                }
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::net_test;
    use super::NetworkManager;
    #[test]
    fn test_network() {
        let mut nm = NetworkManager::new();
        // match nm.get_access_points() {
        //     Ok(data) => {
        //         for d in data {
        //             println!("data: {}", d);
        //         }
        //     }
        //     Err(e) => println!("Error: {:?}", e),
        // }
        match net_test::bluetooth_info() {
            Ok(()) => print!("run fine"),
            Err(e) => print!("error: {:?}", e),
        }
        assert_eq!(1, 2);
    }
}
