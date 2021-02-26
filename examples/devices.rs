//! Example to scan for a short time and then list all known devices.

use libkoompi::system_settings::bluez_api_export::{BluetoothSession, MacAddress};
use std::time::Duration;
use tokio::runtime;
use tokio::time;
const SCAN_DURATION: Duration = Duration::from_secs(4);
async fn do_connection() -> Result<(), eyre::Report> {
    pretty_env_logger::init();

    let (_, session) = BluetoothSession::new().await?;

    // Start scanning for Bluetooth devices, and wait a while for some to be discovered.
    session.start_discovery().await?;
    time::sleep(SCAN_DURATION).await;
    session.stop_discovery().await?;
    // Get the list of all devices which BlueZ knows about.
    let devices = session.get_devices().await?;
    for dev in devices {
        println!(" address: {:#?} name: {:#?} icon: {:#?} \n", dev.mac_address, dev.name, dev.icon);
    }
    Ok(())
}

fn main() {
    let rt = runtime::Runtime::new().unwrap();
    let future = do_connection();
    rt.block_on(future);
}
