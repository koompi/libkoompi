use libkoompi::system_settings::bluez_api_export::BluetoothSession;
use tokio::runtime;

async fn init_adapter() -> Result<(), eyre::Report> {
    let (_, session) = BluetoothSession::new().await?;
    // Get the list of all Bluetooth adapters on the system
    let adapters = session.get_adapters().await?;
    println!("Adapters: {:#?}", adapters);
    for dev in adapters {
        println!("AdapterID: {:?}", dev.id);
    }
    Ok(())
}
fn main() {
    // pretty_env_logger::init();
    let rt = runtime::Runtime::new().unwrap();
    let future = init_adapter();
    let result = rt.block_on(future);
    match result {
        Ok(()) => println!("Run successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
}
