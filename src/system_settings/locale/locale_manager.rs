use std::io::Error;
use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;
use crate::helpers::{get_val_from_keyval, exec_cmd, read_content, write_content, write_content_overwrite};
use super::locale_category::*;

pub const LS_MEASURE_UNITS: [(&str, &str); 3] = [("km_KH.UTF-8", "Metric"), ("en_US.UTF-8", "Imperial US"), ("en_GB.UTF-8", "Imperial UK")];
const LOCALE: &'static str = "locale";
const LOCALE_DEF: &'static str = "localedef";
const LANGUAGE: &'static str = "LANGUAGE";

/// Structure of System-wide Localization Manager
#[derive(Debug, Clone, Default)]
pub struct LocaleManager {
   lang: String,
   language: String,
   lc_numeric: (String, LCNumeric),
   lc_time: (String, LCTime),
   lc_monetary: (String, LCMonetary),
   lc_measure: (String,LCMeasure),
   list_locales: Vec<String>,
   list_langs: HashMap<String, String>,
}

// Public API
impl LocaleManager {
   /// Initialize method
   pub fn new() -> Result<Self, Error> {
      Self::clone_from_etc()?;

      let mut locale_mn = Self::default();
      let Self {
         lang,
         language,
         lc_numeric,
         lc_time,
         lc_monetary,
         lc_measure,
         list_locales,
         ..
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
               } else if line.starts_with(format!("{}", LC_Keywords::LC_MEASUREMENT).as_str()) {
                  lc_measure.0 = get_val_from_keyval(line, None);
                  Self::set_lc_measure(&mut lc_measure.1);
               } 
            });
         },
         Err(err) => return Err(err), // error handling here
      }

      *language = std::env::var(LANGUAGE).unwrap_or(String::new());

      match exec_cmd(LOCALE_DEF, vec!["--list-archive"]) {
         Ok(stdout) => {
            *list_locales = stdout.lines().map(|line| {
               let line_break = line.split('.').collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect::<Vec<String>>();
               let locale = line_break.first().unwrap();
               match line_break.get(1) {
                  Some(charset) => {
                     let uppercase_charset = charset.to_uppercase();
                     let digit_idx = uppercase_charset.chars().position(|c| c.is_numeric()).unwrap();
                     let formatted_charset = uppercase_charset.split_at(digit_idx);
                     format!("{}.{}-{}", locale, formatted_charset.0, formatted_charset.1)
                  },
                  None => locale.to_string()
               }
               
            }).collect();
         },
         Err(err) => return Err(err), // error handling here
      }
      locale_mn.fetch_list_region_lang();
      Ok(locale_mn)
   }

   /// Return current LANG
   pub fn language(&self) -> (&str, &str) {
      match self.list_langs.iter().find(|&(k, _)| *k == self.lang) {
         Some((key, lang)) => (key, lang),
         None => (&self.lang, &self.lang)
      }
   }

   /// Return a list of all prefered languages
   pub fn list_prefered_langs(&self) -> Vec<(&str, &str)> {
      if self.language.is_empty() {
         vec![(&self.lang, self.language().1.split("(").collect::<Vec<&str>>().first().unwrap_or(&self.language().1))]
      } else {
         let ls_lang_reg = self.list_langs.iter().map(|(key, lang)| (key.as_str(), *lang.split("(").collect::<Vec<&str>>().first().unwrap_or(&lang.as_str()))).collect::<Vec<(&str, &str)>>();
         let ls_prefered_langs = self.language.split(":").collect::<Vec<&str>>().iter().map(|lang| format!("{}.UTF-8", lang)).collect::<Vec<String>>();
         // ls_lang_reg.into_iter().filter(|(k, _)| ls_prefered_langs.contains(&k.to_string())).collect()
         ls_prefered_langs.into_iter().map(|lc| ls_lang_reg.iter().find(|(k, _)| *k == lc).unwrap_or(&(self.language())).clone()).collect()
      }
   }

   /// Return a list of all enabled locales formatted as "lang - region (locale)"
   pub fn list_langs_regions(&self) -> &HashMap<String, String> {
      &self.list_langs
   }

   /// Return a list of all enabled locales in raw format
   pub fn list_locales(&self) -> Vec<&str> {
      self.list_locales.iter().map(AsRef::as_ref).collect()
   }

   /// Return current LC_NUMERIC
   pub fn numeric(&self) -> (&String, &String) {
      self.list_langs.iter().find(|(k, _)| *k.to_string() == self.lc_numeric.0).unwrap_or((&self.lc_numeric.0, &self.lc_numeric.0))
   }

   /// Return details of current LC_NUMERIC
   pub fn numeric_details(&self) -> &LCNumeric {
      &self.lc_numeric.1
   }

   /// Return current LC_TIME
   pub fn time(&self) -> (&String, &String) {
      self.list_langs.iter().find(|(k, _)| *k.to_string() == self.lc_time.0).unwrap_or((&self.lc_time.0, &self.lc_time.0))
   }

   /// Return details of current LC_TIME
   pub fn time_details(&self) -> &LCTime {
      &self.lc_time.1
   }

   /// Return current LC_MONETARY
   pub fn monetary(&self) -> (&String, &String) {
      self.list_langs.iter().find(|(k, _)| *k.to_string() == self.lc_monetary.0).unwrap_or((&self.lc_monetary.0, &self.lc_monetary.0))
   }

   /// Return details of current LC_MONETARY
   pub fn monetary_details(&self) -> &LCMonetary {
      &self.lc_monetary.1
   }

   /// Return current LC_MEASUREMENT
   pub fn measurement(&self) -> (&String, &String) {
      self.list_langs.iter().find(|(k, _)| *k.to_string() == self.lc_measure.0).unwrap_or((&self.lc_measure.0, &self.lc_measure.0))
   }

   /// Return details of LC_MEASUREMENT
   pub fn measurement_details(&self) -> &LCMeasure {
      &self.lc_measure.1
   }

   /// Set locale by specified a localeConf struct like locale.conf file structure
   pub fn set_locale(&mut self, lc_conf: LocaleConf) -> Result<(), Error> {
      let data = Self::to_locale_string(lc_conf);
      self.write_local(&data)
      // match target {
      //    ExportTarget::Local => self.write_local(&data),
      //    ExportTarget::Global => {
      //       let p = Path::new("/etc/locale.conf");
      //       if let Err(err) = self.write_local(&data) {
      //          return Err(err);
      //       } 
      //       write_content_overwrite(p, &data)
      //    }
      // }
   }

}

// Private Methods
impl LocaleManager {
   // write locale string to HOME config
   fn write_local(&self, data: &str) -> Result<(), Error> {
      let path = dirs::config_dir().unwrap().join("locale.conf");
      Ok(write_content_overwrite(path, data)?)
   }

   // format locale conf to string
   fn to_locale_string(locale_conf: LocaleConf) -> String {
      format!(
         "LANG={lang}\nLANGUAGE={language}\nLC_NUMERIC={lc_numeric}\nLC_TIME={lc_time}\nLC_MONETARY={lc_monetary}\nLC_MEASUREMENT={lc_measurement}\n",
         lang = locale_conf.lang,
         language = locale_conf.language,
         lc_numeric = locale_conf.lc_numeric,
         lc_time = locale_conf.lc_time,
         lc_monetary = locale_conf.lc_monetary,
         lc_measurement = locale_conf.lc_measurement,
      )
   }

   /// write content from /etc to HOME if not exists
   fn clone_from_etc() -> Result<(), Error> {
      match read_content("/etc/locale.conf") {
         Ok(content) => match write_content(dirs::config_dir().unwrap().join("locale.conf"), &content) {
            Ok(_) => Ok(()),
            Err(err) => Err(err), 
         },
         Err(err) => Err(err)
      }
   }

   /// Fetch all the Language and region of enabled locales
   fn fetch_list_region_lang(&mut self) {
      let mut ls_langs = HashMap::new();
      self.list_locales.iter().for_each(|locale| {
         std::env::set_var("LC_ADDRESS", locale);
         match exec_cmd(LOCALE, vec!["lang_name", "country_name"]) {
            Ok(stdout) => {
               if !stdout.trim().is_empty() {
                  let lang_reg = stdout.lines().map(|line| line.trim()).collect::<Vec<&str>>().join(" — ");
                  ls_langs.insert(locale.to_owned(), format!("{} ({})", lang_reg.trim(), locale));
               } else {
                  ls_langs.insert(locale.to_owned(), locale.to_owned());
               }
            },
            Err(err) => eprintln!("{}", err),
         }
      });
      self.list_langs = ls_langs;
   }

   /// Fetch current locale LC_NUMERIC
   fn set_lc_numeric(lc_numeric: &mut LCNumeric) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_NUMERIC).as_str()]) {
         Ok(stdout) => {
            let re = regex::Regex::new(r"[0-9]+;[0-9]+").unwrap();
            let stdout_formatted = stdout.replace("-", "_").lines().map(|line| re.replace(line, "0")).map(|m| m.to_string()).collect::<Vec<String>>().join("\n");
            *lc_numeric = toml::from_str(&stdout_formatted).unwrap_or_default();
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   /// Fetch current locale LC_TIME
   fn set_lc_time(lc_time: &mut LCTime) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_TIME).as_str()]) {
         Ok(stdout) => {
            let stdout_formatted = stdout.replace("-", "_").lines().filter(|line| !line.starts_with("time_era_entries=")).map(ToString::to_string).fold(Vec::new(), |mut formatted, line| {
               if get_val_from_keyval(line.as_str(), None).is_empty() && !line.contains("\"") {
                  formatted.push(format!("{}{}", line, "\"\""))
               } else {
                  formatted.push(line)
               }
               formatted
            }).join("\n");
            *lc_time = toml::from_str(&stdout_formatted).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   /// Fetch current locale LC_MONETARY
   fn set_lc_monetary(lc_monetary: &mut LCMonetary) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MONETARY).as_str()]) {
         Ok(stdout) => {
            let re = regex::Regex::new(r"[0-9]+;[0-9]+").unwrap();
            let stdout_formatted = stdout.replace("-", "_").lines().map(|line| re.replace(line, "0")).map(|m| m.to_string()).collect::<Vec<String>>().join("\n");
            *lc_monetary = toml::from_str(&stdout_formatted).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   /// Fetch current locale LC_MEASUREMENT
   fn set_lc_measure(lc_measure: &mut LCMeasure) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MEASUREMENT).as_str()]) {
         Ok(stdout) => {
            *lc_measure = toml::from_str(&stdout).unwrap_or_default(); 
         },
         Err(err) => eprintln!("{}", err), 
      }
   }
}

/// Available export target variants
// enum ExportTarget {
//    Local,
//    Global,
// }

/// Structure of LocaleConf (file locale.conf format)
pub struct LocaleConf {
   pub lang: String,
   pub language: String,
   pub lc_numeric: String,
   pub lc_time: String,
   pub lc_monetary: String,
   pub lc_measurement: String,
}

/// List of LC_* variants
#[allow(non_camel_case_types)]
enum LC_Keywords {
   LANG,
   LC_NUMERIC,
   LC_TIME,
   LC_MONETARY,
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
         LC_MEASUREMENT => "LC_MEASUREMENT",
      })
   }
}

#[cfg(test)]
mod test {
   use super::{LocaleManager, LocaleConf};
   #[test]
   fn test_locale_manager() {
      match LocaleManager::new() {
         Ok(mut locale_mn) => {
            let lc_conf = LocaleConf {
               lang: String::from("en_US.UTF-8"),
               language: String::from("km_KH:en_US"),
               lc_numeric: String::from("km_KH.UTF-8"),
               lc_time: String::from("km_KH.UTF-8"),
               lc_monetary: String::from("km_KH.UTF-8"),
               lc_measurement: String::from("km_KH.UTF-8"),
            };

            match locale_mn.set_locale(lc_conf) {
               Ok(()) => println!("Success set locale"),
               Err(err) => eprintln!("Error: {}", err),
            }
            locale_mn.list_prefered_langs().iter().for_each(|(key, lang_reg)| println!("{} => {}", key, lang_reg));
            println!("{:#?}", locale_mn);
            assert_eq!(locale_mn.numeric().1, "ខ្មែរ_កម្ពុជា");
         },
         Err(err) => eprintln!("{}", err)
      }
   }
}