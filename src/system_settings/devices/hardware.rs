use udev::{Devices, Enumerator, Hwdb};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let hardware = udev::Hwdb::new();
        match hardware {
            Ok(h_context) => {}
            Err(e) => {}
        }

        let mut enumerator = udev::Enumerator::new().unwrap();
        for device in enumerator.scan_devices().unwrap() {
            println!();
            println!("initialized: {:?}", device.is_initialized());
            println!("     devnum: {:?}", device.devnum());
            println!("    syspath: {:?}", device.syspath());
            println!("    devpath: {:?}", device.devpath());
            println!("  subsystem: {:?}", device.subsystem());
            println!("    sysname: {:?}", device.sysname());
            println!("     sysnum: {:?}", device.sysnum());
            println!("    devtype: {:?}", device.devtype());
            println!("     driver: {:?}", device.driver());
            println!("    devnode: {:?}", device.devnode());
            if let Some(parent) = device.parent() {
                println!("     parent: {:?}", parent.syspath());
            } else {
                println!("     parent: None");
            }
            println!("  [properties]");
            for property in device.properties() {
                println!("    - {:?} {:?}", property.name(), property.value());
            }
            println!("  [attributes]");
            for attribute in device.attributes() {
                println!("    - {:?} {:?}", attribute.name(), attribute.value());
            }
        }
        assert_eq!(2 + 2, 3);
    }
}
