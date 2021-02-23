use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};

pub fn get_bool_yesno(val: &str) -> bool {
   let trim_val = val.trim();
   if trim_val == "yes" {
      true
   } else {
      false
   }
}

pub fn get_val_from_keyval(line: &str, sep: Option<&str>) -> String {
   match get_list_by_sep(line, sep.unwrap_or("=")).get(1) {
      Some(val) => val.trim().replace("\"", ""),
      None => String::new()
   }
}

/// This function is used to execute a command with arguments and return the result.
pub fn exec_cmd(cmd: &str, args: Vec<&str>) -> Result<String, Error> {
   let output = Command::new(cmd).args(args).output()?; 
   if output.status.success() {
      // match String::from_utf8(output.stdout) {
      //    Ok(stdout) => Ok(stdout),
      //    Err(err) => Err(Error::new(ErrorKind::InvalidData, err))
      // }
      Ok(String::from_utf8_lossy(output.stdout.as_ref()).as_ref().trim().to_owned())
   } else if let Ok(stderr) = String::from_utf8(output.stderr) {
      Err(Error::new(ErrorKind::InvalidData, stderr))
   } else {
      Err(Error::from(ErrorKind::Other))
   }
}

/// This function is used to execute a spawn command with arguments and optionally stdin and return result.
pub fn exec_spawn_cmd(cmd: &str, args: Vec<&str>, stdin: Option<&str>) -> Result<String, Error> {
   use std::io::Write;
   let mut child = Command::new(cmd).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
   if let Some(stdin) = stdin {
      let child_stdin = child.stdin.as_mut().unwrap();
      child_stdin.write_all(stdin.as_bytes())?;
   }   
   let output = child.wait_with_output()?;
   if output.status.success() {
      Ok(String::from_utf8_lossy(output.stdout.as_ref()).as_ref().to_owned())
   } else if let Ok(stderr) = String::from_utf8(output.stderr) {
      Err(Error::new(ErrorKind::InvalidData, stderr))
   } else {
      Err(Error::from(ErrorKind::Other))
   }
}

/// This function is used to execute a pipeline commands and return result.
pub fn exec_pipe_cmd(cmd1: (&str, Vec<&str>), cmd2: (&str, Vec<&str>)) -> Result<String, Error> {
   let mut cmd1_child = Command::new(cmd1.0).args(cmd1.1).stdout(Stdio::piped()).spawn()?; 
   if let Some(cmd1_output) = cmd1_child.stdout.take() {
      let cmd2_child = Command::new(cmd2.0).args(cmd2.1).stdin(cmd1_output).stdout(Stdio::piped()).spawn()?;
      let output = cmd2_child.wait_with_output()?;
      cmd1_child.wait()?;

      if output.status.success() {
         Ok(String::from_utf8_lossy(output.stdout.as_ref()).as_ref().to_owned())
      } else if let Ok(stderr) = String::from_utf8(output.stderr) {
         Err(Error::new(ErrorKind::InvalidData, stderr))
      } else {
         Err(Error::from(ErrorKind::Other))
      }
   } else {
      Err(Error::from(ErrorKind::Other))
   }

}

/// This function is used to split string by delimiter and return list of string after split.
pub fn get_list_by_sep(val: &str, sep: &str) -> Vec<String> {
   val.split(sep).map(ToOwned::to_owned).collect()
}