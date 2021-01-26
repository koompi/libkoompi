#![allow(unused_variables)]
#![allow(unused_imports)]
use udev::{Devices, Enumerator, Hwdb};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let device = udev::Device::from_syspath(std::path::Path::new("/sys/class/bluetooth"));
        // device.iter().enumerate().for_each(|(key, value)| {
        //     println!("{:?} : {:?}", key, value.sysname());
        // });
        // let enum_dev = udev::Enumerator::new().unwrap();

        assert_eq!(2 + 2, 3);
    }
}
