use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn get_bool_yesno(val: &str) -> bool {
   let trim_val = val.trim();
   if trim_val == "yes" {
      true
   } else {
      false
   }
}

pub fn get_val_from_keyval(line: &str, sep: Option<&str>) -> String {
   match line.split(sep.unwrap_or("=")).collect::<Vec<&str>>().get(1) {
      Some(val) => val.trim().to_string(),
      None => String::new()
   }
}

pub fn exec_cmd(cmd: &str, args: Vec<&str>) -> Result<String, Error> {
   let output = Command::new(cmd).args(args).output()?; 
   if output.status.success() {
      // match String::from_utf8(output.stdout) {
      //    Ok(stdout) => Ok(stdout),
      //    Err(err) => Err(Error::new(ErrorKind::InvalidData, err))
      // }
      Ok(String::from_utf8_lossy(output.stdout.as_ref()).as_ref().to_owned())
   } else if let Ok(stderr) = String::from_utf8(output.stderr) {
      Err(Error::new(ErrorKind::InvalidData, stderr))
   } else {
      Err(Error::new(ErrorKind::Other, ""))
   }
}