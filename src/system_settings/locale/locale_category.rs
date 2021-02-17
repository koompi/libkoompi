use serde::{Deserialize};
use std::io::Error;
use crate::helpers::{get_list_by_sep, exec_cmd, get_val_from_keyval};
use super::locale_manager::{LOCALE, LC_Keywords};

/// Structure of LC_NUMERIC
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCNumeric {
   pub decimal_point: String,
   pub thousands_sep: String,
   pub grouping: u8,
}

impl LCNumeric {
   /// Fetch current locale LC_NUMERIC
   pub fn new() -> Result<Self, Error> {
      let stdout = exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_NUMERIC).as_str()])?;
      let re = regex::Regex::new(r"[0-9]+;[0-9]+").unwrap();
      let stdout_formatted = stdout.replace("-", "_").lines().map(|line| re.replace(line, "0")).map(|m| m.to_string()).collect::<Vec<String>>().join("\n");
      Ok(toml::from_str(&stdout_formatted).unwrap_or_default())
   }
}

/// Structure of LC_TIME
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCTime {
   abday: String,
   day: String,
   abmon: String,
   mon: String,
   am_pm: String,
   pub d_t_fmt: String,
   pub d_fmt: String,
   pub t_fmt: String,
   pub t_fmt_ampm: String,
   alt_digits: String,
   pub era: String,
   pub era_d_fmt: String,
   pub era_t_fmt: String,
   pub era_d_t_fmt: String,
   pub first_weekday: u8,
}

impl LCTime {
   /// Fetch current locale LC_TIME
   pub fn new() -> Result<Self, Error> {
      let stdout = exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_TIME).as_str()])?;
      let stdout_formatted = stdout.replace("-", "_").lines().filter(|line| !line.starts_with("time_era_entries=")).map(ToString::to_string).fold(Vec::new(), |mut formatted, line| {
         if get_val_from_keyval(line.as_str(), None).is_empty() && !line.contains("\"") {
            formatted.push(format!("{}{}", line, "\"\""))
         } else {
            formatted.push(line)
         }
         formatted
      }).join("\n");
      Ok(toml::from_str(&stdout_formatted).unwrap_or_default()) 
   }
   /// Return a list of abbreviated days.
   pub fn list_abbr_days(&self) -> Vec<String> {
      get_list_by_sep(self.abday.as_str(), ";")
   }

   /// Return a list of full-formatted days.
   pub fn list_days(&self) -> Vec<String> {
      get_list_by_sep(self.day.as_str(), ";")
   }

   /// Return a list of abbreviated months.
   pub fn list_abbr_months(&self) -> Vec<String> {
      get_list_by_sep(self.abmon.as_str(), ";")
   }

   /// Return a list of full-formatted months.
   pub fn list_months(&self) -> Vec<String> {
      get_list_by_sep(self.mon.as_str(), ";")
   }

   /// Return formatted AM/PM as a list.
   pub fn str_am_pm(&self) -> Vec<String> {
      get_list_by_sep(self.am_pm.as_str(), ";")
   }

   /// Return a list of alternative digits if current locale had the digits else Return None.
   pub fn list_alt_digits(&self) -> Option<Vec<String>> {
      if !self.alt_digits.is_empty() {
         Some(get_list_by_sep(self.alt_digits.as_str(), ";"))
      } else {
         None
      }
   }
}

/// Structure of LC_MONETARY
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMonetary {
   pub int_curr_symbol: String,
   pub currency_symbol: String,
   pub mon_decimal_point: String,
   pub mon_thousands_sep: String,
   pub mon_grouping: u8,
   pub positive_sign: String,
   pub negative_sign: String,
   pub int_frac_digits: u8,
   pub frac_digits: u8,
}

impl LCMonetary {
   /// Fetch current locale LC_MONETARY
   pub fn new() -> Result<Self, Error> {
      let stdout = exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MONETARY).as_str()])?;
      let re = regex::Regex::new(r"[0-9]+;[0-9]+").unwrap();
      let stdout_formatted = stdout.replace("-", "_").lines().map(|line| re.replace(line, "0")).map(|m| m.to_string()).collect::<Vec<String>>().join("\n");
      Ok(toml::from_str(&stdout_formatted).unwrap_or_default()) 
   }
}

/// Structure of LC_MEASUREMENT
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMeasure {
   pub measurement: usize,
}

impl LCMeasure {
   /// Fetch current locale LC_MEASUREMENT
   pub fn new() -> Result<Self, Error> {
      let stdout = exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MEASUREMENT).as_str()])?;
      Ok(toml::from_str(&stdout).unwrap_or_default())
   }
}