#![allow(dead_code)]
use crate::helpers::{get_property, set_device_path};
use dbus::blocking::Connection;
pub struct Brightness {
    device: BDevice,
}

impl Brightness {
    pub fn new() -> Self {
        Self {
            device: BDevice::new(),
        }
    }
    pub fn set_percent(&mut self, percent: u32) {
        self.device.set_bright(percent);
    }
    pub fn get_percent(&self) -> u32 {
        self.device.get_current_level()
    }
    pub fn get_max_percent(&self) -> u32 {
        self.device.get_max_bright()
    }
    pub fn login1_set_brightness(&self, level: u32) -> Result<(), Box<dyn std::error::Error>> {
        match self.device.set_dbus_bright(level) {
            Ok(()) => println!("run fine"),
            Err(e) => println!("Not crash but lovable error: {:?}", e),
        }
        Ok(())
    }
    pub fn information(&self) -> Vec<u32> {
        self.device.info()
    }
}
#[derive(Default)]
struct BDevice {
    id: &'static str,
    class: &'static str,
    max_brightness: u32,
    current_brightness: u32,
}
impl BDevice {
    fn new() -> Self {
        let max_bright = get_property("max_brightness");
        let cur_bright = get_property("brightness");
        Self {
            max_brightness: max_bright,
            current_brightness: cur_bright,
            ..Default::default()
        }
    }
    /// set brightness for dispaly device
    fn set_bright(&mut self, level: u32) {
        match set_device_path("backlight/intel_backlight") {
            Ok(mut dev) => {
                if dev.is_initialized() {
                    println!("device has already been handled by udev")
                }
                // give time for cpu to execute on other processes
                std::thread::sleep(std::time::Duration::from_millis(10));
                let value = self.percent_to_value(level);
                match dev.set_attribute_value("brightness", value.to_string()) {
                    Ok(()) => {
                        self.update(value);
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            Err(e) => eprint!("error: {:?}", e),
        }
    }
    // return roundf(powf(percent / 100, p.exponent) * d->max_brightness);
    // calculate percent value range 0..=100 by by round_up(f32::powf(input_v / 100, 1) * max_value, -2) as u32
    fn percent_to_value(&self, val: u32) -> u32 {
        math::round::ceil(
            (f32::powf(val as f32 / 100.0, 1.0) * self.get_max_bright() as f32) as f64,
            -2,
        ) as u32
    }
    // if (val < 0)
    // 		return 0;
    // 	float ret = powf(val / d->max_brightness, 1.0f / p.exponent) * 100;
    // 	return rnd ? roundf(ret) : ret;
    fn val_to_percent(&self, val: u32, rnd: bool) -> u32 {
        if val.le(&0) {
            1
        } else {
            let ret = f32::powf(val as f32 / self.get_max_bright() as f32, 1.0) * 100.0;
            if rnd {
                math::round::ceil(ret as f64, 2) as u32
            } else {
                ret as u32
            }
        }
    }
    fn get_max_bright(&self) -> u32 {
        self.max_brightness
    }
    // bool logind_set_brightness(struct device *d) {
    //     sd_bus *bus = NULL;
    //     int r = sd_bus_default_system(&bus);
    //     if (r < 0) {
    //         fprintf(stderr, "Can't connect to system bus: %s\n", strerror(-r));
    //         return false;
    //     }
    //     r = sd_bus_call_method(bus,
    //                    "org.freedesktop.login1",
    //                    "/org/freedesktop/login1/session/auto",
    //                    "org.freedesktop.login1.Session",
    //                    "SetBrightness",
    //                    NULL,
    //                    NULL,
    //                    "ssu",
    //                    d->class,
    //                    d->id,
    //                    d->curr_brightness);
    //     if (r < 0)
    //         fprintf(stderr, "Failed to set brightness: %s\n", strerror(-r));
    //     sd_bus_unref(bus);
    //     return r >= 0;
    // }
    fn set_dbus_bright(&self, level: u32) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::new_system()?;
        let proxy = conn.with_proxy(
            "org.freedesktop.login1",
            "/org/freedesktop/login1/session/auto",
            std::time::Duration::from_millis(100),
        );
        let (status,): (bool,) = proxy.method_call(
            "org.freedesktop.login1.Session",
            "SetBrightness",
            ("backlight", "intel_backlight", level),
        )?;
        println!("dubs stats: {:?}", status);
        if status {
            println!("set brigthness success");
        } else {
            println!("failed to set bright nesss");
        }
        Ok(())
    }
    fn get_current_level(&self) -> u32 {
        self.val_to_percent(self.current_brightness, true)
    }
    fn info(&self) -> Vec<u32> {
        vec![self.max_brightness, self.get_current_level()]
    }
    fn update(&mut self, current: u32) {
        self.current_brightness = current;
    }
}

#[cfg(test)]
mod tests {
    use super::Brightness;
    // use std::time::Duration;
    #[test]
    fn it_works() {
        let bright = Brightness::new();
        bright.login1_set_brightness(15000).unwrap();
        // println!("current : {}", bright.get_percent());
        // println!("max_percent: {}", bright.get_max_percent());
        // for i in 1..=10 {
        //     std::thread::sleep(Duration::from_millis(1000));
        //     bright.set_percent(i * 10);
        // }
        // assert_eq!(100, bright.get_percent());
        assert_eq!(2 + 2, 3);
    }
}
