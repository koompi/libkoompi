use libkoompi::system_settings::{DeviceControl, SoundCard, SourceController};
fn main() {
    
    let mut handler = SourceController::create();
    let devices = handler.list_devices().expect("Device not found");
    for dev in devices.clone() {
        println!("{:?} : {:?} : {:?}", dev.index, dev.description, dev.name);
    }
    let mut test_string = String::new();
    println!("Current Volume: {:?}", handler.get_volume());
    std::io::stdin().read_line(&mut test_string).expect("Error reading string. cannot store the input string in the buffer");
    for dev in devices.clone() {
        if test_string.trim() == dev.index.to_string() {
            println!("Device Index: {:?}", dev.index);
            handler.set_device_volume_by_name(&dev.name.unwrap(), 0.40);
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
}
