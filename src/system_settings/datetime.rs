use std::io::Error;
use crate::helpers::{get_bool_yesno, exec_cmd, constants::PKEXEC};
use getset::{Getters};
use std::collections::HashMap;
use itertools::Itertools;

const TIMEDATE_CTL: &str = "timedatectl";

/// Structure of DateTimeManager
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

// Public API
impl DateTimeManager {
   /// Initialize method
   pub fn new() -> Result<Self, Error> {
      let mut datetime_mn = Self::default();
      datetime_mn.load_info()?;
      let stdout = exec_cmd(TIMEDATE_CTL, vec!["list-timezones"])?;
      let mut ls_timezones: Vec<String> = stdout.lines().map(|line| line.trim().to_string()).collect();
      ls_timezones.push(String::from("Asia/Phnom_Penh"));
      ls_timezones.sort();
      datetime_mn.list_timezones = ls_timezones.into_iter().group_by(|tz| tz.split_terminator('/').collect::<Vec<&str>>().iter().map(ToString::to_string).collect::<Vec<String>>()[0].clone()).into_iter().map(|(con, cities)| (con.to_string(), cities.collect::<Vec<String>>().into_iter().map(|city| city.split_terminator('/').collect::<Vec<&str>>().iter().map(ToString::to_string).collect::<Vec<String>>().last().unwrap().clone()).collect())).collect();
      Ok(datetime_mn)
   }

   /// This method is used to set datetime manually.
   pub fn set_datetime(&mut self, datetime: &str) -> Result<bool, Error> {
      if !self.ntp {
         exec_cmd(TIMEDATE_CTL, vec!["set-time", datetime])?;
         self.time_usec = datetime.to_owned();
         self.load_info()?;
         Ok(true)
      } else {
         Ok(false)
      }
   }

   /// This method is used to set timezone manually.
   pub fn set_timezone(&mut self, tz: &str) -> Result<bool, Error> {
      if !self.ntp {
         exec_cmd(PKEXEC, vec!["ln", "-sf", format!("/usr/share/zoneinfo/{}", tz).as_str(), "/etc/localtime"])?;
         self.timezone = tz.to_owned();
         self.load_info()?;
         Ok(true)
      } else {
         Ok(false)
      }
   }

   /// This method is used to enable/disable Network Time Protocol.
   pub fn set_ntp(&mut self, ntp: bool) -> Result<bool, Error> {
      exec_cmd(TIMEDATE_CTL, vec!["set-ntp", format!("{}", ntp).as_str()])?;
      self.ntp = ntp;
      // system clock synchronized
      // self.ntp_sync = ntp;
      self.load_info()?;
      Ok(true)
   }

   /// This method is used to set Local RealTime Clock.
   pub fn set_local_rtc(&mut self, local_rtc: bool) -> Result<bool, Error> {
      if !self.ntp {
         exec_cmd(TIMEDATE_CTL, vec!["set-local-rtc", if local_rtc {"true"} else {"0"}])?;
         self.local_rtc = local_rtc;
         Ok(true)
      } else {
         Ok(false)
      }
   }

   /// This method is to load all info about date time.
   fn load_info(&mut self) -> Result<(), Error> {
      let stdout = exec_cmd(TIMEDATE_CTL, vec!["show"])?;
      stdout.lines().for_each(|line| {
         if line.starts_with("Timezone=") {
            self.timezone = line.split_at(9).1.trim().to_owned();
         } else if line.starts_with("LocalRTC=") {
            if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
               self.local_rtc = get_bool_yesno(val);
            }
         } else if line.starts_with("CanNTP=") {
            if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
               self.can_ntp = get_bool_yesno(val)
            }
         } else if line.starts_with("NTP=") {
            if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
               self.ntp = get_bool_yesno(val)
            }
         } else if line.starts_with("NTPSynchronized=") {
            if let Some(val) = line.split('=').collect::<Vec<&str>>().get(1) {
               self.ntp_sync = get_bool_yesno(val)
            }
         } else if line.starts_with("TimeUSec=") {
            self.time_usec = line.split_at(9).1.trim().to_owned();
         } else if line.starts_with("RTCTimeUSec=") {
            self.rtc_time_usec = line.split_at(12).1.trim().to_owned();
         } 
      });
      Ok(())
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
