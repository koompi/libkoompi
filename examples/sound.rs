use libkoompi::system_settings::{DeviceControl, SinkController, SoundCard};
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
            // handler.decrease_device_volume_by_percent(dev.index, 0.10);
            match handler.get_card_info_list() {
                Ok(list_cards) => {
                    for data in list_cards.iter() {
                        for ports in data.ports.iter() {
                            println!("Port: {:?}", ports.description.as_ref().unwrap());
                        }
                        for profiles_list in data.profiles.iter() {
                            println!("Profile: {:?}", profiles_list.description.as_ref().unwrap());
                        }
                    }
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
    println!("Test sound module");
}
