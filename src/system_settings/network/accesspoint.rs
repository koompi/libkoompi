use dbus::arg::PropMap;
use dbus::blocking::{Connection, Proxy};
use dbus::Error;
use std::collections::HashMap;
use std::time::Duration;
const SERVICE_NAME: &str = "org.freedesktop.NetworkManager";
const SERVICE_INTERFACE: &str = "org.freedesktop.NetworkManager";
const ACESSPOINT_INTERFACE: &str = "org.freedesktop.NetworkManager.AccessPoint";
#[derive(Default, Debug)]
pub struct AccessPoint {
    pub ssid: String,
    pub strenght: u8,
    pub last_seen: i32,
    pub hwaddress: String,
    pub flags: u32,
    pub frequency: u32,
    pub max_bitrate: u32,
    pub mode: u32,
    pub rns_flags: u32,
    pub wpa_flags: u32,
}

pub fn get_accesspoints() -> Result<Vec<AccessPoint>, Error> {
    let mut vec_accesspoint: Vec<AccessPoint> = Vec::new();
    let conn = Connection::new_system()?;
    let mut proxy = conn.with_proxy(SERVICE_NAME, "/org/freedesktop/NetworkManager", Duration::from_millis(2000));
    // Now we make the method call. The ListNames method call take zero input paramters and one output parameter which is an array of strings.Duration
    // Therefore the input is a zero tuple "()" , and the output is a single tuple "(names, "
    let result: Result<Vec<dbus::Path<'static>>, dbus::Error> = proxy.method_call(SERVICE_INTERFACE, "GetDevices", ()).and_then(|r: (Vec<dbus::Path<'static>>,)| Ok(r.0));
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
    for i in &result.unwrap() {
        println!("Path: {:?}", i);
        proxy = conn.with_proxy(SERVICE_NAME, i, Duration::from_millis(1000));
        let dev_type: u32 = proxy.get("org.freedesktop.NetworkManager.Device", "DeviceType")?;
        println!("Device Type Wireless is : {:?}", dev_type);

        if dev_type == 2 {
            let wifi_deice = Proxy::new(SERVICE_NAME, i, Duration::from_millis(1000), &conn);
            let dict: PropMap = HashMap::new();
            let _device: Result<(), Error> = wifi_deice.method_call("org.freedesktop.NetworkManager.Device.Wireless", "RequestScan", (dict,));
            std::thread::sleep(Duration::from_millis(1000));
            let accessspoint: Result<Vec<dbus::Path<'static>>, dbus::Error> = wifi_deice.method_call("org.freedesktop.NetworkManager.Device.Wireless", "GetAccessPoints", ()).and_then(|r: (Vec<dbus::Path<'static>>,)| Ok(r.0));
            accessspoint.unwrap().into_iter().for_each(|access_path| {
                // println!("Path: {:?}", access_path);
                let p = conn.with_proxy(SERVICE_INTERFACE, access_path, Duration::from_millis(2000));
                let data: Result<Vec<u8>, dbus::Error> = p.get(ACESSPOINT_INTERFACE, "Ssid");
                let address: Result<String, dbus::Error> = p.get(ACESSPOINT_INTERFACE, "HwAddress");
                let strength: Result<u8, dbus::Error> = p.get(ACESSPOINT_INTERFACE, "Strength");
                let result_strenght = u8::from_le_bytes([strength.unwrap(); 1]);
                let longdata = &data.unwrap();
                let acutal_data = std::str::from_utf8(longdata);
                vec_accesspoint.push(AccessPoint {
                    ssid: acutal_data.unwrap().to_string(),
                    hwaddress: address.unwrap(),
                    strenght: result_strenght,
                    ..Default::default()
                });
            });
        } else {
            continue;
        }
    }
    Ok(vec_accesspoint)
}
