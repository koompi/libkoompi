pub fn get_bool_yesno(val: &str) -> bool {
   let trim_val = val.trim();
   if trim_val == "yes" {
      true
   } else {
      false
   }
}