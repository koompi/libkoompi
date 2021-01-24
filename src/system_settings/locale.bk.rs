use std::io::Error;
use serde::{Deserialize};
use crate::helpers::{get_val_from_keyval, exec_cmd};
use std::fmt::{self, Display, Formatter};
use getset::Getters;
const LOCALE: &'static str = "locale";
const LOCALE_CTL: &'static str = "localectl";

#[allow(non_camel_case_types)]
pub enum LC_Keywords {
   LANG,
   LC_NUMERIC,
   LC_TIME,
   LC_MONETARY,
   LC_ADDRESS,
   LC_TELEPHONE,
}

impl Display for LC_Keywords {
   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      use LC_Keywords::*;
      write!(f, "{}", match self {
         LANG => "LANG",
         LC_NUMERIC => "LC_NUMERIC",
         LC_TIME => "LC_TIME",
         LC_MONETARY => "LC_MONETARY",
         LC_ADDRESS => "LC_ADDRESS",
         LC_TELEPHONE => "LC_TELEPHONE",
      })
   }
}

#[derive(Debug, Clone, Default, Getters)]
pub struct LocaleManager {
   #[getset(get = "pub")]
   system_locale: SystemLocale,
   #[getset(get = "pub")]
   vc_keymap: String,
   #[getset(get = "pub")]
   vc_toggle_keymap: Option<String>,
   #[getset(get = "pub")]
   x11_layout: String,
   #[getset(get = "pub")]
   x11_model: Option<String>,
   #[getset(get = "pub")]
   x11_variant: Option<String>,
   #[getset(get = "pub")]
   x11_options: Option<String>,
}

impl LocaleManager {
   pub fn new() -> Result<Self, Error> {
      let mut locale_mn = Self::default();
      let Self {
         system_locale: SystemLocale {
            lang,
            lc_numeric,
            lc_time,
            lc_monetary,
            lc_addr,
            lc_tel,
            ls_locale,
         },
         vc_keymap,
         vc_toggle_keymap,
         x11_layout,
         x11_model,
         x11_variant,
         x11_options,
      } = &mut locale_mn;

      let output_num_lines = match exec_cmd(LOCALE_CTL, vec!["status"]) {
         Ok(stdout) => {
            let output_num_lines = stdout.lines().count();
            if output_num_lines <= 4 {
               stdout.lines().map(|line| line.trim()).for_each(|line| {
                  if line.starts_with("VC Keymap:") {
                     *vc_keymap = get_val_from_keyval(line, Some(":"))
                  } else if line.starts_with("VC Toggle Keymap:") {
                     *vc_toggle_keymap = Some(get_val_from_keyval(line, Some(":")))
                  } else if line.starts_with("X11 Layout:") {
                     *x11_layout = get_val_from_keyval(line, Some(":"))
                  } else if line.starts_with("X11 Model:") {
                     *x11_model = Some(get_val_from_keyval(line, Some(":")))
                  } else if line.starts_with("X11 Variant:") {
                     *x11_variant = Some(get_val_from_keyval(line, Some(":")))
                  } else if line.starts_with("X11 Options:") {
                     *x11_options = Some(get_val_from_keyval(line, Some(":")))
                  } 
               });
            } else {
               stdout.lines().map(|line| line.trim()).for_each(|line| {
                  if line.starts_with("System Locale:") {
                     *lang = get_val_from_keyval(line, None);
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_NUMERIC).as_str()) {
                     Self::set_lc_numeric(lc_numeric, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_TIME).as_str()) {
                     Self::set_lc_time(lc_time, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_MONETARY).as_str()) {
                     Self::set_lc_monetary(lc_monetary, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_ADDRESS).as_str()) {
                     Self::set_lc_addr(lc_addr, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_TELEPHONE).as_str()) {
                     Self::set_lc_tel(lc_tel, get_val_from_keyval(line, None));
                  } else if line.starts_with("VC Keymap:") {
                     *vc_keymap = get_val_from_keyval(line, Some(":"))
                  } else if line.starts_with("VC Toggle Keymap:") {
                     *vc_toggle_keymap = Some(get_val_from_keyval(line, Some(":")))
                  } else if line.starts_with("X11 Layout:") {
                     *x11_layout = get_val_from_keyval(line, Some(":"))
                  } else if line.starts_with("X11 Model:") {
                     *x11_model = Some(get_val_from_keyval(line, Some(":")))
                  } else if line.starts_with("X11 Variant:") {
                     *x11_variant = Some(get_val_from_keyval(line, Some(":")))
                  } else if line.starts_with("X11 Options:") {
                     *x11_options = Some(get_val_from_keyval(line, Some(":")))
                  } 
               });
            }
            output_num_lines
         }
         Err(err) => {
            eprintln!("{}", err); // error handling here 
            0
         }, 
      };
      if output_num_lines <= 4 {
         match exec_cmd(LOCALE, Vec::new()) {
            Ok(stdout) => {
               stdout.lines().for_each(|line| {
                  if line.starts_with(format!("{}", LC_Keywords::LANG).as_str()) {
                     *lang = get_val_from_keyval(line, None);
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_NUMERIC).as_str()) {
                     Self::set_lc_numeric(lc_numeric, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_TIME).as_str()) {
                     Self::set_lc_time(lc_time, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_MONETARY).as_str()) {
                     Self::set_lc_monetary(lc_monetary, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_ADDRESS).as_str()) {
                     Self::set_lc_addr(lc_addr, get_val_from_keyval(line, None));
                  } else if line.starts_with(format!("{}", LC_Keywords::LC_TELEPHONE).as_str()) {
                     Self::set_lc_tel(lc_tel, get_val_from_keyval(line, None));
                  } 
               });
            },
            Err(err) => eprintln!("{}", err), // error handling here
         }
      }
      match exec_cmd(LOCALE_CTL, vec!["list-locales"]) {
         Ok(stdout) => {
            *ls_locale = stdout.lines().map(|line| line.to_string()).collect();
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
               system_locale: SystemLocale {
                  lang,
                  lc_numeric,
                  lc_time,
                  lc_monetary,
                  lc_addr,
                  lc_tel,
                  ..
               },
               ..
            } = self;
            let lc = locale.to_string();

            match keyword {
               LANG => *lang = lc,
               LC_NUMERIC => Self::set_lc_numeric(lc_numeric, lc),
               LC_TIME => Self::set_lc_time(lc_time, lc),
               LC_MONETARY => Self::set_lc_monetary(lc_monetary, lc),
               LC_ADDRESS => Self::set_lc_addr(lc_addr, lc),
               LC_TELEPHONE => Self::set_lc_tel(lc_tel, lc),
            }
            Ok(true)
         },
         Err(err) => {
            eprintln!("{}", err);
            Err(err)
         }
      }
   }

   pub fn list_locales(&self) -> &Vec<String> {
      self.system_locale.ls_locale.as_ref()
   }

   pub fn list_keymaps(&self) -> Result<Vec<String>, Error> {
      Ok(exec_cmd(LOCALE_CTL, vec!["list-keymaps"])?.lines().map(ToOwned::to_owned).collect())
   }

   pub fn set_keymap(&mut self, keymap: &str, toggle_map: Option<&str>) -> Result<bool, Error> {
      match exec_cmd(LOCALE_CTL, vec!["set-keymap", keymap, toggle_map.unwrap_or_default()]) {
         Ok(_) => {
            self.vc_keymap = keymap.to_owned();
            self.vc_toggle_keymap = toggle_map.map(ToOwned::to_owned);
            Ok(true)
         },
         Err(err) => Err(err)
      }
   }

   pub fn list_keymap_models(&self) -> Result<Vec<String>, Error> {
      Ok(exec_cmd(LOCALE_CTL, vec!["list-x11-keymap-models"])?.lines().map(ToOwned::to_owned).collect())
   }

   pub fn list_keymap_layouts(&self) -> Result<Vec<String>, Error> {
      Ok(exec_cmd(LOCALE_CTL, vec!["list-x11-keymap-layouts"])?.lines().map(ToOwned::to_owned).collect())
   }

   pub fn list_keymap_variants(&self, layout: &str) -> Result<Vec<String>, Error> {
      Ok(exec_cmd(LOCALE_CTL, vec!["list-x11-keymap-variants", layout])?.lines().map(ToOwned::to_owned).collect())
   }

   pub fn list_keymap_options(&self) -> Result<Vec<String>, Error> {
      Ok(exec_cmd(LOCALE_CTL, vec!["list-x11-keymap-options"])?.lines().map(ToOwned::to_owned).collect())
   }

   pub fn set_x11_keymap(&mut self, layout: &str, model: Option<&str>, variant: Option<&str>, opts: Option<&str>) -> Result<bool, Error> {
      match exec_cmd(LOCALE_CTL, vec!["set-x11-keymap", layout, model.unwrap_or_default(), variant.unwrap_or_default(), opts.unwrap_or_default()]) {
         Ok(_) => {
            self.x11_layout = layout.to_owned();
            self.x11_model = model.map(ToOwned::to_owned);
            self.x11_variant = variant.map(ToOwned::to_owned);
            self.x11_options = opts.map(ToOwned::to_owned);
            Ok(true)
         },
         Err(err) => Err(err)
      }
   }

   fn set_lc_numeric(lc_numeric: &mut LCNumeric, lc: String) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_NUMERIC).as_str()]) {
         Ok(stdout) => {
            *lc_numeric = toml::from_str(&stdout).unwrap_or_default(); 
            lc_numeric.lc = lc;
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_time(lc_time: &mut LCTime, lc: String) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_TIME).as_str()]) {
         Ok(stdout) => {
            let stdout_formatted = stdout.replace("-", "_").lines().filter(|line| !(line.starts_with("era=") || line.starts_with("alt_digits=") || line.starts_with("time_era_entries="))).collect::<Vec<&str>>().join("\n");
            *lc_time = toml::from_str(&stdout_formatted).unwrap_or_default(); 
            lc_time.lc = lc;
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_monetary(lc_monetary: &mut LCMonetary, lc: String) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_MONETARY).as_str()]) {
         Ok(stdout) => {
            let stdout_formatted = stdout.replace("-", "_").lines().filter(|line| !(line.starts_with("mon_grouping=") || line.starts_with("conversion_rate="))).collect::<Vec<&str>>().join("\n");
            *lc_monetary = toml::from_str(&stdout_formatted).unwrap_or_default(); 
            lc_monetary.lc = lc;
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_addr(lc_addr: &mut LCAddress, lc: String) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_ADDRESS).as_str()]) {
         Ok(stdout) => {
            *lc_addr = toml::from_str(&stdout).unwrap_or_default(); 
            lc_addr.lc = lc;
         },
         Err(err) => eprintln!("{}", err), 
      }
   }

   fn set_lc_tel(lc_tel: &mut LCTelephone, lc: String) {
      match exec_cmd(LOCALE, vec!["-k", format!("{}", LC_Keywords::LC_TELEPHONE).as_str()]) {
         Ok(stdout) => {
            *lc_tel = toml::from_str(&stdout).unwrap_or_default(); 
            lc_tel.lc = lc;
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
            match locale_mn.set_keymap("us", None) {
               Ok(is_sucess) => {
                  if is_sucess {
                     println!("Success set keymap")
                  } else {
                     println!("Can not set keymap")
                  }
               },
               Err(err) => eprintln!("{}", err)
            }
            match locale_mn.set_x11_keymap("us", None, None, None) {
               Ok(is_sucess) => {
                  if is_sucess {
                     println!("Success set x11 keymap")
                  } else {
                     println!("Can not set x11 keymap")
                  }
               },
               Err(err) => eprintln!("{}", err)
            }
            assert_eq!(locale_mn.vc_keymap(), "us");
            assert_eq!(locale_mn.x11_layout(), "us");
            assert_eq!(locale_mn.system_locale().lang(), "km_KH.UTF-8");
         },
         Err(err) => eprintln!("{}", err)
      }
   }
}

#[derive(Debug, Clone, Default, Getters)]
pub struct SystemLocale {
   #[getset(get = "pub")]
   lang: String,
   #[getset(get = "pub")]
   lc_numeric: LCNumeric,
   #[getset(get = "pub")]
   lc_time: LCTime,
   #[getset(get = "pub")]
   lc_monetary: LCMonetary,
   #[getset(get = "pub")]
   lc_addr: LCAddress,
   #[getset(get = "pub")]
   lc_tel: LCTelephone,
   #[getset(get = "pub")]
   ls_locale: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCNumeric {
   #[serde(skip)]
   pub lc: String,
   #[serde(alias = "decimal_point")]
   pub dec_point: String,
   pub thousands_sep: String,
   #[serde(alias = "grouping")]
   pub grp: u8,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCTime {
   #[serde(skip)]
   pub lc: String,
   #[serde(alias = "abday")]
   pub ab_day: String,
   pub day: String,
   #[serde(alias = "abmon")]
   pub ab_mon: String,
   pub mon: String,
   pub am_pm: String,
   pub d_t_fmt: String,
   pub d_fmt: String,
   pub t_fmt: String,
   pub t_fmt_ampm: String,
   // era: String,
   pub era_year: String,
   pub era_d_fmt: String,
   pub era_t_fmt: String,
   pub era_d_t_fmt: String,
   // alt_digits: String,
   // #[serde(alias = "time-era-num-entries")]
   pub time_era_num_entries: u8,
   // #[serde(alias = "time-era-entries")]
   // time_era_entries: String,
   // #[serde(alias = "week-ndays")]
   pub week_ndays: u8,
   // #[serde(alias = "week-1stday")]
   pub week_1stday: u32,
   // #[serde(alias = "week-1stweek")]
   pub week_1stweek: u8,
   pub first_weekday: u8,
   pub first_workday: u8,
   pub cal_direction: u8,
   pub timezone: String,
   pub date_fmt: String,
   pub alt_mon: String,
   pub ab_alt_mon: String,
}

// #[derive(Debug, Clone, Default, Deserialize)]
// pub struct LCCollate {
//    #[serde(skip)]
//    lc: String,
//    #[serde(alias = "collate-nrules")]
//    collate_nrules: u8,
//    #[serde(alias = "collate-rulesets")]
//    collate_rulesets: String,
//    #[serde(alias = "collate-symb-hash-sizemb")]
//    collate_symb_hash_sizemb: u32,
// }

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMonetary {
   #[serde(skip)]
   pub lc: String,
   pub int_curr_symbol: String,
   #[serde(alias = "currency_symbol")]
   pub curr_symbol: String,
   #[serde(alias = "mon_decimal_point")]
   pub mon_dec_point: String,
   pub mon_thousands_sep: String,
   // #[serde(alias = "mon_grouping")]
   // mon_grp: String,
   #[serde(alias = "positive_sign")]
   pub pos_sign: String,
   #[serde(alias = "negative_sign")]
   pub neg_sign: String,
   pub int_frac_digits: u8,
   pub frac_digits: u8,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCAddress {
   #[serde(skip)]
   pub lc: String,
   pub postal_fmt: String,
   pub country_name: String,
   pub country_post: String,
   pub country_ab2: String,
   pub country_ab3: String,
   pub country_car: String,
   pub country_num: u16,
   pub country_isbn: String,
   pub lang_name: String,
   pub lang_ab: String,
   pub lang_term: String,
   pub lang_lib: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCTelephone {
   #[serde(skip)]
   pub lc: String,
   pub tel_int_fmt: String,
   pub tel_dom_fmt: String,
   pub int_select: String,
   pub int_prefix: String,
}