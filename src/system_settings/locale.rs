use std::io::Error;
use serde::{Deserialize};
use crate::helpers::{get_val_from_keyval, exec_cmd, get_list_by_sep};
use std::fmt::{self, Display, Formatter};
const LOCALE: &'static str = "locale";
const LOCALE_CTL: &'static str = "localectl";

#[allow(non_camel_case_types)]
pub enum LC_Keywords {
   LANG,
   LC_NUMERIC,
   LC_TIME,
   LC_MONETARY,
   LC_MESSAGES,
   LC_ADDRESS,
   LC_MEASUREMENT,
}

impl Display for LC_Keywords {
   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      use LC_Keywords::*;
      write!(f, "{}", match self {
         LANG => "LANG",
         LC_NUMERIC => "LC_NUMERIC",
         LC_TIME => "LC_TIME",
         LC_MONETARY => "LC_MONETARY",
         LC_MESSAGES => "LC_MESSAGES",
         LC_ADDRESS => "LC_ADDRESS",
         LC_MEASUREMENT => "LC_MEASUREMENT",
      })
   }
}

#[derive(Debug, Clone, Default)]
pub struct LocaleManager {
   lang: String,
   lc_numeric: (String, LCNumeric),
   lc_time: (String, LCTime),
   lc_monetary: (String, LCMonetary),
   lc_messages: (String, LCMessages),
   lc_addr: (String, LCAddress),
   lc_measure: (String,LCMeasure),
   list_locales: Vec<String>,
}

impl LocaleManager {
   pub fn new() -> Result<Self, Error> {
      let mut locale_mn = Self::default();
      let Self {
         lang,
         lc_numeric,
         lc_time,
         lc_monetary,
         lc_messages,
         lc_addr,
         lc_measure,
         list_locales,
      } = &mut locale_mn;

      match exec_cmd(LOCALE, Vec::new()) {
         Ok(stdout) => {
            stdout.lines().for_each(|line| {
               if line.starts_with(format!("{}", LC_Keywords::LANG).as_str()) {
                  *lang = get_val_from_keyval(line, None);
               } else if line.starts_with(format!("{}", LC_Keywords::LC_NUMERIC).as_str()) {
                  lc_numeric.0 = get_val_from_keyval(line, None);
                  Self::set_lc_numeric(&mut lc_numeric.1);
               } else if line.starts_with(format!("{}", LC_Keywords::LC_TIME).as_str()) {
                  lc_time.0 = get_val_from_keyval(line, None);
                  Self::set_lc_time(&mut lc_time.1);
               } else if line.starts_with(format!("{}", LC_Keywords::LC_MONETARY).as_str()) {
                  lc_monetary.0 = get_val_from_keyval(line, None);
                  Self::set_lc_monetary(&mut lc_monetary.1);
               } else if line.starts_with(format!("{}", LC_Keywords::LC_MESSAGES).as_str()) {
                  lc_messages.0 = get_val_from_keyval(line, None);
                  Self::set_lc_messages(&mut lc_messages.1);
               } else if line.starts_with(format!("{}", LC_Keywords::LC_ADDRESS).as_str()) {
                  lc_addr.0 = get_val_from_keyval(line, None);
                  Self::set_lc_addr(&mut lc_addr.1);
               } else if line.starts_with(format!("{}", LC_Keywords::LC_MEASUREMENT).as_str()) {
                  lc_measure.0 = get_val_from_keyval(line, None);
                  Self::set_lc_measure(&mut lc_measure.1);
               } 
            });
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }

      match exec_cmd(LOCALE_CTL, vec!["list-locales"]) {
         Ok(stdout) => {
            *list_locales = stdout.lines().map(|line| line.to_string()).collect();
         },
         Err(err) => eprintln!("{}", err), // error handling here
      }
      Ok(locale_mn)
   }

   pub fn set_locale(&mut self, keyword: LC_Keywords, locale: &str) -> Result<bool, Error> {
      match exec_cmd(LOCALE_CTL, vec!["set-locale", format!("{}={}", keyword, locale).as_str()]) {
         Ok(_) => {
            use LC_Keywords::*;
            let Self {
               lang,
               lc_numeric,
               lc_time,
               lc_monetary,
               lc_messages,
               lc_addr,
               lc_measure,
               ..
            } = self;
            let lc = locale.to_string();

            match keyword {
               LANG => *lang = lc,
               LC_NUMERIC => {
                  lc_numeric.0 = lc;
                  Self::set_lc_numeric(&mut lc_numeric.1);
               },
               LC_TIME => {
                  lc_time.0 = lc;
                  Self::set_lc_time(&mut lc_time.1);
               },
               LC_MONETARY => {
                  lc_monetary.0 = lc;
                  Self::set_lc_monetary(&mut lc_monetary.1);
               },
               LC_MESSAGES => {
                  lc_messages.0 = lc;
                  Self::set_lc_messages(&mut lc_messages.1);
               }
               LC_ADDRESS => {
                  lc_addr.0 = lc;
                  Self::set_lc_addr(&mut lc_addr.1);
               },
               LC_MEASUREMENT => {
                  lc_measure.0 = lc;
                  Self::set_lc_measure(&mut lc_measure.1)
               },
            }
            Ok(true)
         },
         Err(err) => {
            eprintln!("{}", err);
            Err(err)
         }
      }
   }

   pub fn language(&self) -> &str {
      self.lang.as_str()
   }

   pub fn list_locales(&self) -> Vec<&str> {
      self.list_locales.iter().map(AsRef::as_ref).collect()
   }

   pub fn numeric(&self) -> &str {
      self.lc_numeric.0.as_str()
   }

   pub fn numeric_details(&self) -> &LCNumeric {
      &self.lc_numeric.1
   }

   pub fn time(&self) -> &str {
      self.lc_time.0.as_str()
   }

   pub fn time_details(&self) -> &LCTime {
      &self.lc_time.1
   }

   pub fn monetary(&self) -> &str {
      self.lc_monetary.0.as_str()
   }

   pub fn monetary_details(&self) -> &LCMonetary {
      &self.lc_monetary.1
   }

   pub fn messages(&self) -> &str {
      self.lc_messages.0.as_str()
   }

   pub fn messages_details(&self) -> &LCMessages {
      &self.lc_messages.1
   }

   pub fn address(&self) -> &str {
      self.lc_addr.0.as_str()
   }

   pub fn address_details(&self) -> &LCAddress {
      &self.lc_addr.1
   }

   pub fn measurement(&self) -> &str {
      self.lc_measure.0.as_str()
   }

   pub fn measurement_details(&self) -> &LCMeasure {
      &self.lc_measure.1
   }
}

impl LocaleManager {
   fn set_lc_numeric(lc_numeric: &mut LCNumeric) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_NUMERIC).as_str()]) {
         Ok(stdout) => {
            *lc_numeric = toml::from_str(&stdout).unwrap_or_default();
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_time(lc_time: &mut LCTime) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_TIME).as_str()]) {
         Ok(stdout) => {
            let stdout_formatted = stdout.replace("-", "_").lines().filter(|line| !(line.starts_with("era=") || line.starts_with("alt_digits=") || line.starts_with("time_era_entries="))).collect::<Vec<&str>>().join("\n");
            *lc_time = toml::from_str(&stdout_formatted).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_monetary(lc_monetary: &mut LCMonetary) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MONETARY).as_str()]) {
         Ok(stdout) => {
            let stdout_formatted = stdout.replace("-", "_").lines().filter(|line| !(line.starts_with("mon_grouping=") || line.starts_with("conversion_rate="))).collect::<Vec<&str>>().join("\n");
            *lc_monetary = toml::from_str(&stdout_formatted).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_messages(lc_messages: &mut LCMessages) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MESSAGES).as_str()]) {
         Ok(stdout) => {
            *lc_messages = toml::from_str(&stdout).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_addr(lc_addr: &mut LCAddress) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_ADDRESS).as_str()]) {
         Ok(stdout) => {
            *lc_addr = toml::from_str(&stdout).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_measure(lc_measure: &mut LCMeasure) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MEASUREMENT).as_str()]) {
         Ok(stdout) => {
            *lc_measure = toml::from_str(&stdout).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }
}

#[cfg(test)]
mod test {
   use super::{LocaleManager, LC_Keywords};
   #[test]
   fn test_locale_manager() {
      match LocaleManager::new() {
         Ok(mut locale_mn) => {
            match locale_mn.set_locale(LC_Keywords::LANG, "km_KH.UTF-8") {
               Ok(is_sucess) => {
                  if is_sucess {
                     println!("Success set locale {}", LC_Keywords::LANG)
                  } else {
                     println!("Can not set locale {}", LC_Keywords::LANG)
                  }
               },
               Err(err) => eprintln!("{}", err)
            }
            assert_eq!(locale_mn.language(), "km_KH.UTF-8");
         },
         Err(err) => eprintln!("{}", err)
      }
   }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCNumeric {
   pub decimal_point: String,
   pub thousands_sep: String,
   pub grouping: String,
}

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
   pub fn list_abbr_days(&self) -> Vec<String> {
      get_list_by_sep(self.abday.as_str(), ";")
   }

   pub fn list_days(&self) -> Vec<String> {
      get_list_by_sep(self.day.as_str(), ";")
   }

   pub fn list_abbr_months(&self) -> Vec<String> {
      get_list_by_sep(self.abmon.as_str(), ";")
   }

   pub fn list_months(&self) -> Vec<String> {
      get_list_by_sep(self.mon.as_str(), ";")
   }

   pub fn str_am_pm(&self) -> Vec<String> {
      get_list_by_sep(self.am_pm.as_str(), ";")
   }

   pub fn list_alt_digits(&self) -> Vec<String> {
      get_list_by_sep(self.alt_digits.as_str(), ";")
   }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMonetary {
   pub int_curr_symbol: String,
   pub currency_symbol: String,
   pub mon_decimal_point: String,
   pub mon_thousands_sep: String,
   pub mon_grouping: String,
   pub positive_sign: String,
   pub negative_sign: String,
   pub int_frac_digits: u8,
   pub frac_digits: u8,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMessages {
   pub yesexpr: String,
   pub noexpr: String,
   pub yesstr: String,
   pub nostr: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCAddress {
   pub country_name: String,
   pub country_post: String,
   pub country_ab2: String,
   pub country_ab3: String,
   pub lang_name: String,
   pub lang_ab: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMeasure {
   pub measurement: usize,
}