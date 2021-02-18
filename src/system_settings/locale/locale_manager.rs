use std::io::Error;
use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;
use crate::helpers::{get_val_from_keyval, exec_cmd, read_content, write_content, write_content_overwrite};
use super::locale_category::*;

pub const LS_MEASURE_UNITS: [(&str, &str); 3] = [("km_KH.UTF-8", "Metric"), ("en_US.UTF-8", "Imperial US"), ("en_GB.UTF-8", "Imperial UK")];
pub(super) const LOCALE: &str = "locale";
pub(super) const LOCALE_DEF: &str = "localedef";

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
   tmp_locale: LocaleConf,
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

      let mut stdout = exec_cmd(LOCALE, Vec::new())?;
      stdout.lines().for_each(|line| {
         if line.starts_with(format!("{}", LC_Keywords::LANG).as_str()) {
            *lang = get_val_from_keyval(line, None);
         } else if line.starts_with(format!("{}", LC_Keywords::LC_NUMERIC).as_str()) {
            lc_numeric.0 = get_val_from_keyval(line, None);
            lc_numeric.1 = LCNumeric::new().unwrap_or_default();
         } else if line.starts_with(format!("{}", LC_Keywords::LC_TIME).as_str()) {
            lc_time.0 = get_val_from_keyval(line, None);
            lc_time.1 = LCTime::new().unwrap_or_default();
         } else if line.starts_with(format!("{}", LC_Keywords::LC_MONETARY).as_str()) {
            lc_monetary.0 = get_val_from_keyval(line, None);
            lc_monetary.1 = LCMonetary::new().unwrap_or_default();
         } else if line.starts_with(format!("{}", LC_Keywords::LC_MEASUREMENT).as_str()) {
            lc_measure.0 = get_val_from_keyval(line, None);
            lc_measure.1 = LCMeasure::new().unwrap_or_default();
         } 
      });

      *language = std::env::var(format!("{}", LC_Keywords::LANGUAGE)).unwrap_or(String::new());

      stdout = exec_cmd(LOCALE_DEF, vec!["--list-archive"])?;
      *list_locales = stdout.lines().filter(|line| line.contains(".utf8")).map(|line| {
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

   /// write_conf by specified a localeConf struct like locale.conf file structure
   pub fn write_conf(&mut self) -> Result<(), Error> {
      self.tmp_to_val();
      let data = self.to_locale_string();
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

   pub fn set_locale(&mut self, key: LC_Keywords, locale: &str) -> Result<(), Error> {
      std::env::set_var(key.to_string(), locale);
      use LC_Keywords::*;
      match key {
         LANG => self.tmp_locale.lang = locale.to_owned(),
         LANGUAGE => self.tmp_locale.language = locale.to_owned(),
         LC_NUMERIC => {
            self.tmp_locale.lc_numeric = locale.to_owned();
            self.lc_numeric.1 = LCNumeric::new().unwrap_or_default();
         },
         LC_TIME => {
            self.tmp_locale.lc_time = locale.to_owned();
            self.lc_time.1 = LCTime::new().unwrap_or_default();
         },
         LC_MONETARY => {
            self.tmp_locale.lc_monetary = locale.to_owned();
            self.lc_monetary.1 = LCMonetary::new().unwrap_or_default();
         },
         LC_MEASUREMENT => {
            self.tmp_locale.lc_measure = locale.to_owned();
            self.lc_measure.1 = LCMeasure::new().unwrap_or_default();
         }
      }
      Ok(())
   }
}

// Private Methods
impl LocaleManager {
   // write locale string to HOME config
   fn write_local(&self, data: &str) -> Result<(), Error> {
      let path = dirs_next::config_dir().unwrap().join("locale.conf");
      Ok(write_content_overwrite(path, data)?)
   }

   // format locale conf to string
   fn to_locale_string(&self) -> String {
      format!(
         "LANG={lang}\nLANGUAGE={language}\nLC_NUMERIC={lc_numeric}\nLC_TIME={lc_time}\nLC_MONETARY={lc_monetary}\nLC_MEASUREMENT={lc_measurement}\n",
         lang = self.lang,
         language = self.language,
         lc_numeric = self.lc_numeric.0,
         lc_time = self.lc_time.0,
         lc_monetary = self.lc_monetary.0,
         lc_measurement = self.lc_measure.0,
      )
   }

   /// write content from /etc to HOME if not exists
   fn clone_from_etc() -> Result<(), Error> {
      match read_content("/etc/locale.conf") {
         Ok(content) => match write_content(dirs_next::config_dir().unwrap().join("locale.conf"), &content) {
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

   fn tmp_to_val(&mut self) {
      self.lang = self.tmp_locale.lang.to_owned();
      self.language = self.tmp_locale.language.to_owned();
      self.lc_numeric.0 = self.tmp_locale.lc_numeric.to_owned();
      self.lc_time.0 = self.tmp_locale.lc_time.to_owned();
      self.lc_monetary.0 = self.tmp_locale.lc_monetary.to_owned();
      self.lc_measure.0 = self.tmp_locale.lc_measure.to_owned();
   }
}

/// List of LC_* variants
#[allow(non_camel_case_types)]
pub enum LC_Keywords {
   LANG,
   LANGUAGE,
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
         LANGUAGE => "LANGUAGE",
         LC_NUMERIC => "LC_NUMERIC",
         LC_TIME => "LC_TIME",
         LC_MONETARY => "LC_MONETARY",
         LC_MEASUREMENT => "LC_MEASUREMENT",
      })
   }
}

#[cfg(test)]
mod test {
   use super::{LocaleManager, LC_Keywords};
   #[test]
   fn test_locale_manager() {
      match LocaleManager::new() {
         Ok(mut locale_mn) => {
            match locale_mn.set_locale(LC_Keywords::LC_TIME, "km_KH.UTF-8") {
               Ok(()) => {
                  println!("{:#?}", locale_mn.time_details());
                  match locale_mn.write_conf() {
                     Ok(()) => println!("Success set locale"),
                     Err(err) => eprintln!("Error: {}", err),
                  }
               },
               Err(err) => eprintln!("Error: {}", err),
            }
            
            locale_mn.list_prefered_langs().iter().for_each(|(key, lang_reg)| println!("{} => {}", key, lang_reg));
            // println!("{:#?}", locale_mn);
            assert_eq!(locale_mn.numeric().1, "ខ្មែរ_កម្ពុជា");
         },
         Err(err) => eprintln!("{}", err)
      }
   }
}

// Available export target variants
// enum ExportTarget {
//    Local,
//    Global,
// }

/// Structure of LocaleConf (file locale.conf format)
#[derive(Debug, Clone, Default)]
pub struct LocaleConf {
   pub lang: String,
   pub language: String,
   pub lc_numeric: String,
   pub lc_time: String,
   pub lc_monetary: String,
   pub lc_measure: String,
   // pub first_weekday: u8,
   // pub is_24_hour: bool,
}