// pa_context_state_t
use super::callbacks::{server_info_cb, sink_list_cb, source_list_cb, state_cb, success_cb};
use super::device::Device;
use libpulse_sys::context::flags::PA_CONTEXT_NOFLAGS;
use libpulse_sys::context::introspect::{
    pa_context_get_sink_info_list, pa_context_get_source_info_list,
};
use libpulse_sys::context::pa_context;
use libpulse_sys::context::pa_context_connect;
use libpulse_sys::context::pa_context_new;
use libpulse_sys::context::pa_context_set_state_callback;
use libpulse_sys::context::{
    pa_context_get_server_info, pa_context_get_sink_info_by_index,
    pa_context_get_sink_info_by_name, pa_context_get_source_info_by_index,
    pa_context_get_source_info_by_name, pa_context_set_sink_mute_by_index,
    pa_context_set_sink_volume_by_index, pa_context_set_source_mute_by_index,
    pa_context_set_source_volume_by_index,
};

use libpulse_sys::def::pa_device_type_t;
use libpulse_sys::mainloop::api::pa_mainloop_api;
use libpulse_sys::mainloop::standard::pa_mainloop;
use libpulse_sys::mainloop::standard::pa_mainloop_iterate;
use libpulse_sys::operation::pa_operation_unref;
use libpulse_sys::operation::{pa_operation, PA_OPERATION_RUNNING};
use libpulse_sys::pa_operation_get_state;
use libpulse_sys::standard::{pa_mainloop_get_api, pa_mainloop_new};
use libpulse_sys::volume::pa_volume_t;
use libpulse_sys::volume::{pa_cvolume, pa_cvolume_set, PA_VOLUME_MAX};
use std::ffi::c_void;
use std::os::raw::c_int;
use std::ptr::{null, null_mut};
#[repr(C)]
pub struct ServerInfo {
    pub default_source_name: *const i8,
    pub default_sink_name: *const i8,
}
impl ServerInfo {
    pub fn new() -> Self {
        Self {
            default_sink_name: null(),
            default_source_name: null(),
        }
    }
}
#[derive(PartialEq)]
pub enum State {
    CONNECTING,
    CONNECTED,
    ERROR,
}
type state_t = State;

const UINT32_MAX: u32 = u32::MAX;
#[repr(C)]
pub struct Pulseaudio {
    mainloop: *const pa_mainloop,
    mainloop_api: *const pa_mainloop_api,
    context: *mut pa_context,
    retval: c_int,
    pub state: state_t,
}
impl Pulseaudio {
    fn iterate(&mut self, op: *mut pa_operation) {
        unsafe {
            while pa_operation_get_state(op) == PA_OPERATION_RUNNING {
                pa_mainloop_iterate(
                    self.mainloop as *mut pa_mainloop,
                    1,
                    self.retval as *mut c_int,
                );
            }
        }
    }
    pub fn new() -> Self {
        Self {
            mainloop: null_mut(),
            mainloop_api: null_mut(),
            context: null_mut(),
            retval: 0,
            state: state_t::CONNECTED,
        }
    }
    pub fn pulseaudio(&mut self, client_name: *const i8) -> Result<(), std::io::Error> {
        unsafe {
            self.mainloop = pa_mainloop_new();
            self.mainloop_api = pa_mainloop_get_api(self.mainloop);
            self.context = pa_context_new(self.mainloop_api, client_name);
            pa_context_set_state_callback(self.context, Some(state_cb), null_mut());
            self.state = state_t::CONNECTING;
            if pa_context_connect(self.context, null_mut(), PA_CONTEXT_NOFLAGS, null()) < 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Connection Error\n",
                ));
            } else {
                while self.state == state_t::CONNECTING {
                    if pa_mainloop_iterate(
                        self.mainloop as *mut pa_mainloop,
                        1,
                        self.retval as *mut c_int,
                    ) < 0
                    {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Mainloop Error",
                        ));
                    }
                }
                if self.state == state_t::ERROR {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Connection Error",
                    ));
                }
            }
        }
        Ok(())
    }
    pub fn get_sinks(&mut self) -> Vec<Device> {
        let mut sinks: Vec<Device> = Vec::new();
        // let sink_vec: *mut c_void = sinks;
        unsafe {
            let op: *mut pa_operation = pa_context_get_sink_info_list(
                self.context,
                Some(sink_list_cb),
                sinks.as_mut_ptr() as *mut c_void,
            );
            self.iterate(op);
            pa_operation_unref(op);
        }
        sinks
    }
    pub fn get_sources(&mut self) -> Vec<Device> {
        let mut sources: Vec<Device> = Vec::new();
        // let sink_vec: *mut c_void = sinks;
        unsafe {
            let op: *mut pa_operation = pa_context_get_source_info_list(
                self.context,
                Some(source_list_cb),
                sources.as_mut_ptr() as *mut c_void,
            );
            self.iterate(op);
            pa_operation_unref(op);
        }
        sources
    }
    pub fn get_sink(&mut self, index: u32) -> Result<Device, std::io::Error> {
        let mut sinks: Vec<Device> = Vec::new();
        unsafe {
            let op: *mut pa_operation = pa_context_get_sink_info_by_index(
                self.context,
                index,
                Some(sink_list_cb),
                sinks.as_mut_ptr() as *mut c_void,
            );
            pa_operation_unref(op);
        }
        // if sinks.is_empty() {
        //     return Err(std::io::Error::new(
        //         std::io::ErrorKind::InvalidData,
        //         "The sink doesn't exits\n",
        //     ));
        // } else {
        Ok(*sinks.first().unwrap())
        // }
    }
    pub fn get_sink_name(&mut self, name: *const i8) -> Result<Device, std::io::Error> {
        let mut sinks: Vec<Device> = Vec::new();
        unsafe {
            let op: *mut pa_operation = pa_context_get_sink_info_by_name(
                self.context,
                name,
                Some(sink_list_cb),
                sinks.as_mut_ptr() as *mut c_void,
            );
            pa_operation_unref(op);
        }
        if sinks.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "The sink doesn't exits\n",
            ));
        } else {
            Ok(*sinks.first().unwrap())
        }
    }
    pub fn get_source(&mut self, index: u32) -> Result<Device, std::io::Error> {
        let mut sources: Vec<Device> = Vec::new();
        unsafe {
            let op: *mut pa_operation = pa_context_get_source_info_by_index(
                self.context,
                index,
                Some(source_list_cb),
                sources.as_mut_ptr() as *mut c_void,
            );
            pa_operation_unref(op);
        }
        if sources.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "The source doesn't exits\n",
            ));
        } else {
            Ok(*sources.first().unwrap())
        }
    }
    pub fn get_source_name(&mut self, name: *const i8) -> Result<Device, std::io::Error> {
        let mut sources: Vec<Device> = Vec::new();
        unsafe {
            let op: *mut pa_operation = pa_context_get_source_info_by_name(
                self.context,
                name,
                Some(source_list_cb),
                sources.as_mut_ptr() as *mut c_void,
            );
            pa_operation_unref(op);
        }
        if sources.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "The source doesn't exits\n",
            ));
        } else {
            Ok(*sources.first().unwrap())
        }
    }
    pub fn get_default_sink(&mut self) -> Device {
        let mut info = ServerInfo::new();
        let info_ptr: *mut c_void = &mut info as *mut _ as *mut c_void;
        unsafe {
            let op: *mut pa_operation =
                pa_context_get_server_info(self.context, Some(server_info_cb), info_ptr);
            self.iterate(op);
        }
        return self.get_sink_name(info.default_sink_name).unwrap();
    }
    pub fn get_default_source(&mut self) -> Device {
        let mut info = ServerInfo::new();
        let info_ptr: *mut c_void = &mut info as *mut _ as *mut c_void;
        unsafe {
            let op: *mut pa_operation =
                pa_context_get_server_info(self.context, Some(server_info_cb), info_ptr);
            self.iterate(op);
        }
        return self.get_source_name(info.default_source_name).unwrap();
    }
    pub fn set_volume(&mut self, device: &mut Device, new_volume: *mut pa_volume_t) {
        if unsafe { *new_volume > PA_VOLUME_MAX } {
            unsafe {
                *new_volume = PA_VOLUME_MAX;
            }
        } else {
            unsafe {
                let new_cvolume: *mut pa_cvolume = pa_cvolume_set(
                    &mut device.volume as *mut pa_cvolume,
                    device.volume.channels as u32,
                    *new_volume,
                );
                // let mut op: *mut pa_operation = null_mut();
                let op: *mut pa_operation = if device.type_t == pa_device_type_t::Sink {
                    pa_context_set_sink_volume_by_index(
                        self.context,
                        device.index,
                        new_cvolume,
                        Some(success_cb),
                        null_mut(),
                    )
                } else {
                    pa_context_set_source_volume_by_index(
                        self.context,
                        device.index,
                        new_cvolume,
                        Some(success_cb),
                        null_mut(),
                    )
                };
                self.iterate(op);
                pa_operation_unref(op);
            }
        }
    }
    pub fn set_mute(&mut self, device: &Device, mute: bool) {
        unsafe {
            let op: *mut pa_operation = if device.type_t == pa_device_type_t::Sink {
                pa_context_set_sink_mute_by_index(
                    self.context,
                    device.index,
                    mute as i32,
                    Some(success_cb),
                    null_mut(),
                )
            } else {
                pa_context_set_source_mute_by_index(
                    self.context,
                    device.index,
                    mute as i32,
                    Some(success_cb),
                    null_mut(),
                )
            };
            self.iterate(op);
            pa_operation_get_state(op);
        }
    }
}
