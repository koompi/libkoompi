// use clap::{App, Arg, ArgMatches};
// use libkoompi::system_settings::sounds::{device::*, pulseaudio::*};
// use libpulse_sys::volume::{pa_volume_t, PA_VOLUME_NORM};
// use math::round;
// use std::os::raw::{c_char, c_double, c_int};
// use std::ptr::{null, null_mut};
// use std::str::FromStr;
// fn get_selected_device(
//     pulse: &mut Pulseaudio,
//     args: &ArgMatches,
//     sink_name: *const c_char,
//     source_name: *const c_char,
// ) -> Device {
//     let mut device: Device = pulse.get_default_sink();
//     if args.is_present("sink") {
//         device = pulse.get_sink_name(sink_name).unwrap();
//     } else if args.is_present("default-source") {
//         device = pulse.get_default_source();
//     } else if args.is_present("source") {
//         device = pulse.get_source_name(source_name).unwrap();
//     }
//     return device;
// }
// fn gammaCorrection(i: pa_volume_t, gamma: c_double, delta: c_int) -> pa_volume_t {
//     let mut j: c_double = i as c_double;
//     let rel_relta: c_double = delta as c_double / 100.0;
//     j = j / PA_VOLUME_NORM as f64;
//     j = f64::powf(j, 1.0 / gamma);
//     j = j + rel_relta;
//     if j < 0.0 {
//         j = 0.0;
//     }
//     j = f64::powf(j, gamma);
//     j = j * PA_VOLUME_NORM as f64;
//     return round::ceil(j, -2) as pa_volume_t;
// }
// fn main() {
//     let sink_name: *const c_char = null();
//     let source_name: *const c_char = null();
//     let mut value: c_int;
//     let mut limit_value: c_int;
//     let gamma: c_double;

//     let matches = App::new("My Program")
//         .version("1.0")
//         .author("Ma veasna")
//         .about("Sound Settings")
//         .arg(
//             Arg::with_name("sink")
//                 .long("sink")
//                 .help("choose a different sink than the default")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("source")
//                 .long("source")
//                 .help("choose a different sink than the default")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("default-source")
//                 .long("default-source")
//                 .help("select the default source")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("get-volume")
//                 .long("get-volume")
//                 .help("get the current volume")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("get-volume-human")
//                 .long("get-volume-human")
//                 .help("get the current volume percentage or the string \"muted\"")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("set-volume")
//                 .long("set-volume")
//                 .help("set the volume")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("increase")
//                 .short("i")
//                 .long("increase")
//                 .help("increase the volume")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("decrease")
//                 .short("d")
//                 .long("decrease")
//                 .help("decrease the volume")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("toggle-mute")
//                 .short("t")
//                 .long("toggle-mute")
//                 .help("toggle the volume")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("mute")
//                 .long("mute")
//                 .help("set mute")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("allow-boost")
//                 .long("allow-boost")
//                 .help("allow volume to go above 100%")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("set-limit")
//                 .long("set-limit")
//                 .help("set a limit for the volume")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("gamma")
//                 .long("gamma")
//                 .help("increase/decrease using gamma correction e.g 2.2")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("unmute")
//                 .long("unmute")
//                 .help("unset mute")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("get-mute")
//                 .long("get-mute")
//                 .help("set a limit for the volume")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("list-sinks")
//                 .long("list-sinks")
//                 .help("list the sinks")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("list-sources")
//                 .long("list-sources")
//                 .help("list the sources")
//                 .takes_value(true),
//         )
//         .get_matches();
//     value = 70;
//     gamma = 0.5;
//     let mut pulse: Pulseaudio = Pulseaudio::new();
//     let client: *const i8 = "pamixer".as_ptr() as *const i8;
//     pulse.pulseaudio(client).unwrap();
//     let mut device: Device = get_selected_device(&mut pulse, &matches, sink_name, source_name);
//     if matches.is_present("set-volume")
//         | matches.is_present("increase")
//         | matches.is_present("decrease")
//     {
//         if value < 0 {
//             value = 0;
//         }
//         let mut new_volume: pa_volume_t = 0;
//         new_volume = if matches.is_present("set-volume") {
//             println!("this block run");
//             round::ceil((value as c_double) * PA_VOLUME_NORM as c_double, -2) as u32
//         } else if matches.is_present("increase") {
//             gammaCorrection(device.volume_avg, gamma, value)
//         } else if matches.is_present("decrease") {
//             gammaCorrection(device.volume_avg, gamma, -value)
//         } else {
//             0
//         };
//         if !matches.is_present("allow-boost") && new_volume > PA_VOLUME_NORM {
//             new_volume = PA_VOLUME_NORM;
//         }
//         pulse.set_volume(&mut device, new_volume as *mut u32);
//         device = get_selected_device(&mut pulse, &matches, sink_name, source_name);
//         println!("new value: {}", new_volume);
//         println!("Device: {:?}", device.volume_avg);
//     }
// }

fn main() {
    println!("No Sound detected");
}
