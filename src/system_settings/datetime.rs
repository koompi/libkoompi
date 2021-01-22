use std::io::Error;
use crate::helpers::{get_bool_yesno, exec_cmd};
use getset::Getters;

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
      match exec_cmd(TIMEDATE_CTL, vec!["list-timezones"]) {
         Ok(stdout) => {
            datetime_mn.list_timezones = stdout.lines().map(|line| line.to_string()).collect();
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
         match exec_cmd(TIMEDATE_CTL, vec!["set-timezone", tz]){
            Ok(_) => {
               self.timezone = tz.to_owned();
               res = true;
            },
            Err(err) => eprintln!("{}", err), // error handling here
         }
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
}

#[cfg(test)]
mod tests {
   use super::DateTimeManager;

   #[test]
   fn it_works() {
      match DateTimeManager::new() {
         Ok(mut dt_mn) => {
            if let Ok(res) = dt_mn.set_ntp(false) {
               if res {
                  assert_eq!(*dt_mn.ntp(), false)
               }
            }
         },
         Err(err) => println!("{}", err)
      }
   }
}