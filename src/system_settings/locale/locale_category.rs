use crate::helpers::get_list_by_sep;
use serde::Deserialize;

/// Structure of LC_NUMERIC
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCNumeric {
   pub decimal_point: String,
   pub thousands_sep: String,
   pub grouping: u8,
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

/// Structure of LC_MESSAGES
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMessages {
   pub yesexpr: String,
   pub noexpr: String,
   pub yesstr: String,
   pub nostr: String,
}

/// Structure of LC_ADDRESS
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCAddress {
   pub country_name: String,
   pub country_post: String,
   pub country_ab2: String,
   pub country_ab3: String,
   pub lang_name: String,
   pub lang_ab: String,
}

/// Structure of LC_MEASUREMENT
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LCMeasure {
   pub measurement: usize,
}
