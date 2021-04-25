use std::io::Error;
use std::process::{Command, Stdio};

pub fn exec_cmd(program: &str, args: Vec<&str>) -> Result<bool, Error> {
    let process = Command::new(program).args(args).spawn();
    // match process {
    //     Ok(output) => {
    //         if String::from_utf8_lossy(&output.stdout).contains("Connected: yes") {
    //             Ok(true)
    //         } else {
    //             Ok(false)
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {:?}", e);
    //         Ok(false)
    //     }
    // }
    match process {
        Ok(child) => match child.wait_with_output() {
            Ok(output) => {
                if String::from_utf8_lossy(&output.stdout).contains("Connected: yes") {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}
fn process_scan(program: &str, args: Vec<&str>) {
    let scan_process = Command::new(program).args(args).spawn();
    match scan_process {
        Ok(mut child) => {
            child.try_wait();
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }
}
fn main() {
    process_scan("bluetoothctl", ["scan", "on"].to_vec());
    println!("On");
    std::thread::sleep(std::time::Duration::from_secs(4));
    println!("off");
    process_scan("bluetoothctl", ["scan", "off"].to_vec());
    exec_cmd("bluetoothctl", ["connect", "48:95:07:C6:27:5C"].to_vec());
    // match exec_cmd("bluetoothctl") {
    //     Ok(status) => {
    //         println!("Connection Success: {}", status);
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {:?}", e)
    //     }
    // }
}
