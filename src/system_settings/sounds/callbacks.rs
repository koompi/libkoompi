use std::ffi::c_void;

use super::device::Device;
use super::pulseaudio::ServerInfo;
use super::pulseaudio::{Pulseaudio, State};
use libpulse_sys::context::introspect::{pa_server_info, pa_sink_info, pa_source_info};
use libpulse_sys::context::pa_context;
use libpulse_sys::context::pa_context_get_state;
use libpulse_sys::context::{
    PA_CONTEXT_AUTHORIZING, PA_CONTEXT_CONNECTING, PA_CONTEXT_FAILED, PA_CONTEXT_READY,
    PA_CONTEXT_SETTING_NAME, PA_CONTEXT_TERMINATED, PA_CONTEXT_UNCONNECTED,
};
// extern "C" {
//     pub fn state_cb(context: *mut pa_context, raw: *mut c_void);
//     pub fn sink_list_cb(context: *mut pa_context, i: &pa_sink_info, eol: i32, raw: *mut c_void);

//     pub fn source_list_cb(context: *mut pa_context, i: &pa_source_info, eol: i32, raw: *mut c_void);
//     pub fn server_info_cb(conext: *mut pa_context, i: *const pa_server_info, raw: *mut c_void);

//     pub fn success_cb(context: *mut pa_context, sucess: i32, raw: c_void);

// }

pub extern "C" fn state_cb(context: *mut pa_context, raw: *mut c_void) {
    let mut pulse = Pulseaudio::new();
    match unsafe { pa_context_get_state(context) } {
        PA_CONTEXT_READY => {
            pulse.state = State::CONNECTED;
        }
        PA_CONTEXT_FAILED => {
            pulse.state = State::ERROR;
        }
        PA_CONTEXT_UNCONNECTED
        | PA_CONTEXT_AUTHORIZING
        | PA_CONTEXT_SETTING_NAME
        | PA_CONTEXT_CONNECTING
        | PA_CONTEXT_TERMINATED => {}
    }
}
pub extern "C" fn sink_list_cb(
    context: *mut pa_context,
    i: *const pa_sink_info,
    eol: i32,
    raw: *mut c_void,
) {
    if eol != 0 {
        return;
    } else {
        let sinks: *mut Vec<Device> = raw as *mut Vec<Device>;
        let mut s: Device = Device::new();
        s.DeviceSink(i);
        unsafe {
            (*sinks).push(s);
        }
    }
}

pub extern "C" fn source_list_cb(
    context: *mut pa_context,
    i: *const pa_source_info,
    eol: i32,
    raw: *mut c_void,
) {
    if eol != 0 {
        return;
    } else {
        let sources: *mut Vec<Device> = raw as *mut Vec<Device>;
        let mut s: Device = Device::new();
        s.DeviceSource(i);
        unsafe {
            (*sources).push(s);
        }
    }
}
pub extern "C" fn server_info_cb(
    conext: *mut pa_context,
    i: *const pa_server_info,
    raw: *mut c_void,
) {
    let info: *mut ServerInfo = raw as *mut ServerInfo;
    let mut info1 = ServerInfo::new();
    unsafe {
        info1.default_sink_name = (*i).default_sink_name;
        info1.default_source_name = (*i).default_source_name;
    }
}

pub extern "C" fn success_cb(context: *mut pa_context, sucess: i32, raw: *mut c_void) {
    unimplemented!();
}
