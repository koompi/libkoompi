use std::io::Error;
use std::process::Command;
use crate::helpers::get_bool_yesno;
use getset::{Getters, MutGetters};

#[derive(Debug, Clone, Getters, MutGetters)]
pub struct DateTimeManager {
   #[getset(get = "pub")]
   timezone: String,
   local_rtc: bool,
   can_ntp: bool,
   #[getset(get = "pub")]
   ntp: bool,
   ntp_sync: bool,
   #[getset(get = "pub")]
   time_usec: String,
   rtc_time_usec: String,
   #[getset(get = "pub")]
   list_timezones: Vec<String>,
}

impl Default for DateTimeManager {
   fn default() -> Self {
      Self {
         timezone: String::default(),
         local_rtc: false,
         can_ntp: true,
         ntp: true,
         ntp_sync: true,
         time_usec: String::default(),
         rtc_time_usec: String::default(),
         list_timezones: Vec::new()
      }
   }
}

impl DateTimeManager {
   pub fn new() -> Result<Self, Error> {
      let mut datetime_mn = Self::default();
      let output = Command::new("timedatectl").arg("show").output()?;
      if output.status.success() {
         match String::from_utf8(output.stdout) {
            Ok(stdout) => {
               stdout.lines().for_each(|line| {
                  if line.starts_with("Timezone=") {
                     datetime_mn.timezone = line.split_at(9).1.trim().to_owned();
                  } else if line.starts_with("LocalRTC=") {
                     if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
                        datetime_mn.local_rtc = get_bool_yesno(val);
                     }
                  } else if line.starts_with("CanNTP=") {
                     if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
                        datetime_mn.can_ntp = get_bool_yesno(val)
                     }
                  } else if line.starts_with("NTP=") {
                     if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
                        datetime_mn.ntp = get_bool_yesno(val)
                     }
                  } else if line.starts_with("NTPSynchronized=") {
                     if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
                        datetime_mn.ntp_sync = get_bool_yesno(val)
                     }
                  } else if line.starts_with("TimeUSec=") {
                     datetime_mn.time_usec = line.split_at(9).1.trim().to_owned();
                  } else if line.starts_with("RTCTimeUSec=") {
                     datetime_mn.rtc_time_usec = line.split_at(12).1.trim().to_owned();
                  } 
               });
            }
            Err(err) => eprintln!("{}", err), // error handling here
         }
      }
      
      let output = Command::new("timedatectl").arg("list-timezones").output()?;
      if output.status.success() {
         match String::from_utf8(output.stdout) {
            Ok(stdout) => {
               datetime_mn.list_timezones = stdout.lines().map(|line| line.to_string()).collect();
            },
            Err(err) => eprintln!("{}", err), // error handling here
         }
      }
      Ok(datetime_mn)
   }

   pub fn set_datetime(&mut self, datetime: &str) -> Result<bool, Error> {
      let mut res = false;
      match Command::new("timedatectl").arg("set-time").arg(datetime).output() {
         Ok(output) => {
            if output.status.success() {
               self.time_usec = datetime.to_owned();
               res = true;
            } else if let Ok(stderr) = String::from_utf8(output.stderr) {
               eprintln!("{}", stderr); // error handling here
            }
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }
      Ok(res)
   }

   pub fn set_timezone(&mut self, tz: &str) -> Result<bool, Error> {
      let mut res = false;
      match Command::new("timedatectl").arg("set-timezone").arg(tz).output() {
         Ok(output) => {
            if output.status.success() {
               self.timezone = tz.to_owned();
               res = true;
            } else if let Ok(stderr) = String::from_utf8(output.stderr) {
               eprintln!("{}", stderr); // error handling here
            }
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }
      Ok(res)
   }

   pub fn set_ntp(&mut self, ntp: bool) -> Result<bool, Error> {
      let mut res = false;
      match Command::new("timedatectl").arg("set-ntp").arg(format!("{}", ntp)).output() {
         Ok(output) => {
            if output.status.success() {
               self.ntp = ntp;

               // system clock synchronized
               // self.ntp_sync = ntp;
               res = true;
            } else if let Ok(stderr) = String::from_utf8(output.stderr) {
               eprintln!("{}", stderr); // error handling here
            }
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }
      Ok(res)
   }

   pub fn set_local_rtc(&mut self, local_rtc: bool) -> Result<bool, Error> {
      let mut res = false;
      match Command::new("timedatectl").arg("set-local-rtc").arg(format!("{}", local_rtc)).output() {
         Ok(output) => {
            if output.status.success() {
               self.local_rtc = local_rtc;
               res = true;
            } else if let Ok(stderr) = String::from_utf8(output.stderr) {
               eprintln!("{}", stderr); // error handling here
            }
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }
      Ok(res)
   }
}

#[cfg(test)]
mod tests {
   use crate::system_settings::DateTimeManager;

   #[test]
   fn it_works() {
      match DateTimeManager::new() {
         Ok(mut dt_mn) => {
            if let Ok(res) = dt_mn.set_ntp(true) {
               if res {
                  assert_eq!(*dt_mn.ntp(), true)
               }
            }
         },
         Err(err) => println!("{}", err)
      }
   }
}