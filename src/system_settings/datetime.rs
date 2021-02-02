use std::io::Error;
use crate::helpers::{get_bool_yesno, exec_cmd};
use getset::{Getters};
use std::collections::HashMap;
use itertools::Itertools;

const TIMEDATE_CTL: &'static str = "timedatectl";

#[derive(Debug, Clone, Getters)]
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
   list_timezones: HashMap<String, Vec<String>>,
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
         list_timezones: HashMap::new(),
      }
   }
}

impl DateTimeManager {
   pub fn new() -> Result<Self, Error> {
      let mut datetime_mn = Self::default();
      Self::load_info(&mut datetime_mn);
      match exec_cmd(TIMEDATE_CTL, vec!["list-timezones"]) {
         Ok(stdout) => {
            let mut ls_timezones: Vec<String> = stdout.lines().map(|line| line.trim().to_string()).collect();
            ls_timezones.push(String::from("Asia/Phnom_Penh"));
            ls_timezones.sort();
            datetime_mn.list_timezones = ls_timezones.into_iter().group_by(|tz| tz.split_terminator('/').collect::<Vec<&str>>().iter().map(ToString::to_string).collect::<Vec<String>>()[0].clone()).into_iter().map(|(con, cities)| (con.to_string(), cities.collect::<Vec<String>>())).collect();
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }
      Ok(datetime_mn)
   }

   pub fn set_datetime(&mut self, datetime: &str) -> Result<bool, Error> {
      let mut res = false;
      if !self.ntp {
         match exec_cmd(TIMEDATE_CTL, vec!["set-time", datetime]) {
            Ok(_) => {
               self.time_usec = datetime.to_owned();
               Self::load_info(self);
               res = true;
            },
            Err(err) => eprintln!("{}", err), // error handling here
         }
      }
      Ok(res)
   }

   pub fn set_timezone(&mut self, tz: &str) -> Result<bool, Error> {
      let mut res = false;
      if !self.ntp {
         // println!("{:?}", vec!["ln", "-sf", format!("/usr/share/zoneinfo/{}", tz).as_str(), "/etc/localtime"]);
         match exec_cmd("pkexec", vec!["ln", "-sf", format!("/usr/share/zoneinfo/{}", tz).as_str(), "/etc/localtime"]){
            Ok(_) => {
               self.timezone = tz.to_owned();
               Self::load_info(self);
               res = true;
            },
            Err(err) => eprintln!("{}", err), // error handling here
         }

         // let output = std::process::Command::new("pkexec").arg("timedatectl").arg("set-timezone").arg(tz).output()?;

         // if output.status.success() {
         //    self.timezone = tz.to_owned();
         //    Self::load_info(self);
         //    res = true;
         // }
         // else if let Ok(stderr) = String::from_utf8(output.stderr) {
         //    return Err(Error::new(ErrorKind::Other, stderr));
         // }
      }
      Ok(res)
   }

   pub fn set_ntp(&mut self, ntp: bool) -> Result<bool, Error> {
      let mut res = false;
      match exec_cmd(TIMEDATE_CTL, vec!["set-ntp", format!("{}", ntp).as_str()]) {
         Ok(_) => {
            self.ntp = ntp;
            // system clock synchronized
            // self.ntp_sync = ntp;
            Self::load_info(self);
            res = true;
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }
      Ok(res)
   }

   pub fn set_local_rtc(&mut self, local_rtc: bool) -> Result<bool, Error> {
      let mut res = false;
      if !self.ntp {
         match exec_cmd(TIMEDATE_CTL, vec!["set-local-rtc", if local_rtc {"true"} else {"0"}]) {
            Ok(_) => {
               self.local_rtc = local_rtc;
               res = true;
            },
            Err(err) => eprintln!("{}", err), // error handling here
         }
      }
      Ok(res)
   }
   fn load_info(datetime_mn: &mut DateTimeManager) {
      match exec_cmd(TIMEDATE_CTL, vec!["show"]) {
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
}

#[cfg(test)]
mod tests {
   use super::DateTimeManager;

   #[test]
   fn test_dt_manager() {
      match DateTimeManager::new() {
         Ok(mut dt_mn) => {
            if let Ok(res) = dt_mn.set_ntp(false) {
               if res {
                  assert_eq!(*dt_mn.ntp(), false)
               }
            }

            if let Ok(res) = dt_mn.set_timezone("Asia/Phnom_Penh") {
               if res {
                  println!("{}", *dt_mn.timezone());
               }
            } 
            println!("{:#?}", dt_mn.list_timezones());
            assert_eq!(*dt_mn.timezone(), "Phnom_Penh");
         },
         Err(err) => println!("{}", err)
      }
   }
}
