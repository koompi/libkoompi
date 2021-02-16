use std::process::Command;
use std::{fmt, io};
pub trait Connectivity: fmt::Debug {
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError>;
    fn disconnect(&mut self, ssid: &str) -> Result<bool, WifiConnectionError>;
}

pub trait WifiInterface: fmt::Debug {
    /// Check if the wifi interface on host machine is enabled.
    fn is_wifi_enabled() -> Result<bool, WifiError> {
        unimplemented!();
    }

    /// Turn on the wifi interface of host machine.
    fn turn_on() -> Result<(), WifiError> {
        unimplemented!();
    }

    /// Turn off the wifi interface of host machine.
    fn turn_off() -> Result<(), WifiError> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub enum WifiConnectionError {
    /// Adding the newtork profile failed.
    #[cfg(target_os = "windows")]
    AddNetworkProfileFailed,
    /// Failed to connect to wireless network.
    FailedToConnect(String),
    /// Failed to disconnect from wireless network. Try turning the wireless interface down.
    FailedToDisconnect(String),
    /// A wireless error occurred.
    Other { kind: WifiError },
    // SsidNotFound,
}

impl From<io::Error> for WifiConnectionError {
    fn from(error: io::Error) -> Self {
        WifiConnectionError::Other { kind: WifiError::IoError(error) }
    }
}

#[derive(Debug)]
pub enum WifiError {
    // The specified wifi  is currently disabled. Try switching it on.
    WifiDisabled,
    /// The wifi interface interface failed to switch on.
    #[cfg(target_os = "windows")]
    InterfaceFailedToOn,
    /// IO Error occurred.
    IoError(io::Error),
}
#[derive(Debug, Default)]
pub struct Wifi;
impl WifiInterface for Wifi {
    fn is_wifi_enabled() -> Result<bool, WifiError> {
        let output = Command::new("nmcli").args(&["radio", "wifi"]).output().map_err(|err| WifiError::IoError(err))?;

        Ok(String::from_utf8_lossy(&output.stdout).replace(" ", "").replace("\n", "").contains("enabled"))
    }
    fn turn_on() -> Result<(), WifiError> {
        Command::new("nmcli").args(&["radio", "wifi", "on"]).output().map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }
    fn turn_off() -> Result<(), WifiError> {
        Command::new("nmcli").args(&["radio", "wifi", "off"]).output().map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }
}
impl Connectivity for Wifi {
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError> {
        if !Wifi::is_wifi_enabled().map_err(|err| WifiConnectionError::Other { kind: err })? {
            return Err(WifiConnectionError::Other { kind: WifiError::WifiDisabled });
        }
        let output = Command::new("nmcli")
            .args(&["d", "wifi", "connect", ssid, "password", &password])
            .output()
            .map_err(|err| WifiConnectionError::FailedToConnect(format!("{}", err)))?;
        if !String::from_utf8_lossy(&output.stdout).as_ref().contains("successfully activated") {
            Ok(false)
        } else {
            Ok(true)
        }
    }
    fn disconnect(&mut self, ssid: &str) -> Result<bool, WifiConnectionError> {
        let output = Command::new("nmcli").args(&["connection", "down", ssid]).output().map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;
        if !String::from_utf8_lossy(&output.stdout).as_ref().contains("successfully deactivated") {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

#[test]
fn test_wifi() -> Result<(), WifiConnectionError> {
    let mut wifi = Wifi::default();
    println!("turn off wifi: {:?}", Wifi::turn_on());
    println!("is wifi enable :{:?}", Wifi::is_wifi_enabled());
    match wifi.connect("Koompi OS", "Hi@Koompi1") {
        Ok(is_success) => println!("Connection success: {}", is_success),
        Err(e) => println!("Error: {:?}", e),
    }
    assert_eq!(1, 2);
    Ok(())
}
