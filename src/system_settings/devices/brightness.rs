#![allow(dead_code)]
use crate::helpers::{get_property, set_device_path};
use udev::Enumerator;
pub struct Brightness {
    device: BDevice,
}

impl Brightness {
    pub fn set_current(&mut self) {}
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
    pub fn set_bright(&mut self, level: u32) {
        match set_device_path("backlight/intel_backlight") {
            Ok(mut dev) => {
                if dev.is_initialized() {
                    println!("device has already been handled by udev")
                }
                // give time for cpu to execute on other processes
                std::thread::sleep(std::time::Duration::from_millis(10));
                match dev.set_attribute_value("brightness", level.to_string()) {
                    Ok(()) => {
                        self.update(level);
                        println!("value to percent: {:?}", self.val_to_percent(level, true));
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
    pub fn percent_to_value(&self, val: u32) -> u32 {
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
    pub fn get_max_bright(&self) -> u32 {
        self.max_brightness
    }

    pub fn info(&self) -> Vec<u32> {
        vec![self.max_brightness, self.current_bright()]
    }
    fn update(&mut self, current: u32) {
        self.current_brightness = current;
    }
    pub fn current_bright(&self) -> u32 {
        self.val_to_percent(self.current_brightness, true)
    }
}

#[cfg(test)]
mod tests {
    use super::BDevice;
    #[test]
    fn it_works() {
        let mut bright = BDevice::new();
        println!("current : {}", bright.current_bright());
        assert_eq!(100, bright.current_bright());
        assert_eq!(24000, bright.percent_to_value(100));
        for i in 1..=10 {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            bright.set_bright(2400 * i);
        }
        assert_eq!(2 + 2, 4);
    }
}
