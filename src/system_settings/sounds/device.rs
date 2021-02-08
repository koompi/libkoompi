use libpulse_sys::context::introspect::{pa_sink_info, pa_source_info};
use libpulse_sys::def::pa_device_type_t;
use libpulse_sys::def::pa_device_type_t::{Sink, Source};
use libpulse_sys::volume::{pa_cvolume, pa_cvolume_avg, pa_volume_t, PA_VOLUME_NORM};
use std::ptr::null;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Device {
    pub index: u32,
    pub type_t: pa_device_type_t,
    pub name: *const i8,
    pub description: *const i8,
    pub volume: pa_cvolume,
    pub volume_avg: pa_volume_t,
    pub volume_percent: i32,
    pub mute: bool,
}
impl Device {
    pub fn new() -> Self {
        Self {
            index: 0,
            type_t: pa_device_type_t::Sink,
            name: null(),
            description: null(),
            volume: pa_cvolume::default(),
            volume_avg: pa_volume_t::default(),
            volume_percent: 0,
            mute: false,
        }
    }
    pub fn DeviceSource(&mut self, info: *const pa_source_info) {
        self.type_t = Source;
        unsafe {
            self.index = (*info).index;
            self.name = (*info).name;
            self.description = (*info).description;
            self.mute = (*info).mute == 1;
            self.setVolume(&(*info).volume);
        }
    }
    pub fn DeviceSink(&mut self, info: *const pa_sink_info) {
        self.type_t = Sink;
        unsafe {
            self.index = (*info).index;
            self.name = (*info).name;
            self.description = (*info).description;
            self.mute = (*info).mute == 1;
            self.setVolume(&(*info).volume);
        }
    }
    pub fn setVolume(&mut self, v: *const pa_cvolume) {
        unsafe {
            self.volume = *v;
        }
        self.volume_avg = unsafe { pa_cvolume_avg(v) };
        self.volume_percent =
            math::round::ceil((self.volume_avg * 100 / PA_VOLUME_NORM) as f64, -2) as i32;
    }
}
