use libkoompi::system_settings::{AppControl, DeviceControl, SinkController, SoundCard};
use std::io::Read;
fn main() {
    let mut handler = SinkController::create();
    let devices = handler.list_devices().expect("Device not found");

    for dev in devices.clone() {
        println!("{:?} : {:?} : {:?}", dev.index, dev.description, dev.name);
    }
    let mut test_string = String::new();
    std::io::stdin().read_line(&mut test_string).expect("Error reading string. cannot store the input string in the buffer");
    for dev in devices.clone() {
        if test_string.trim() == dev.index.to_string() {
            handler.decrease_device_volume_by_percent(dev.index, 0.10);
            match handler.get_sound_card_by_index(dev.index) {
                Ok(card_info) => {
                    println!("Card info: {:?} {:?} {:?} Proplist: {:?}", card_info.index, card_info.name, card_info.driver, card_info.proplist);
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
    println!("Test sound module");
}
