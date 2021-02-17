mod account_type;

pub use account_type::AccountType;

use std::fmt::Display;
use std::io::Error;
use super::super::users_groups::{ADM_GROUP, PASSWD, GPASSWD};
use crate::helpers::{exec_cmd, get_list_by_sep};

const USER_ADD: &str = "useradd";
const USER_MOD: &str = "usermod";
const USER_DEL: &str = "userdel";
const GROUPS: &str = "groups";

#[derive(Debug, Clone, Default)]
pub struct User {
   uid: u16,
   gid: u16,
   acc_type: AccountType,
   fullname: String,
   usrname: String,
   login_shell: String,
   home_dir: String,
}

impl User {
   pub fn new<T: AsRef<str>>(fullname: T, usrname: T, account_type: AccountType, pwd: T, verify_pwd: T) -> Result<(), Error> {
      exec_cmd(USER_ADD, vec!["-c", fullname.as_ref(), "-N", usrname.as_ref()])?;
      if account_type == AccountType::Admin {
         exec_cmd(GPASSWD, vec!["-a", usrname.as_ref(), ADM_GROUP])?;
      }
      Ok(())
   }

   pub fn from_vec<T: AsRef<str> + Display>(fields: &[T]) -> Self {
      let mut iter = fields.iter();
      let usrname = iter.next().unwrap().to_string();
      iter.next().unwrap();
      let uid: u16 = iter.next().unwrap().to_string().parse().unwrap();
      let gid: u16 = iter.next().unwrap().to_string().parse().unwrap();
      let fname = iter.next().unwrap().to_string();
      let fullname = if fname.is_empty() {usrname.clone()} else {fname};
      let home_dir = iter.next().unwrap();
      let login_shell = iter.next().unwrap();

      Self {
         uid, gid, 
         fullname: fullname.to_string(), 
         usrname: usrname.to_string(), 
         login_shell: login_shell.to_string(), 
         home_dir: home_dir.to_string(), 
         acc_type: AccountType::default()
      }
   }

   pub fn change_account_type(&mut self, account_type: AccountType) -> Result<String, Error> {
      let opt = match account_type {
         AccountType::Normal => "-d",
         AccountType::Admin => "-a",
      };
      exec_cmd(GPASSWD, vec![opt, self.username(), ADM_GROUP])
   }

   pub fn change_info<T: AsRef<str>>(&mut self, uid: Option<u16>, gname: Option<T>, fullname: T, login_name: Option<T>, login_shell: T, home_dir: Option<T>) -> Result<(), Error> {
      let mut args = Vec::new();
      if let Some(uid) = uid {
         args.extend(vec!["-u", &format!("{}", uid)]);
      } if let Some(gname) = &gname {
         args.extend(vec!["-g", gname.as_ref()]);
      } if let Some(login_name) = &login_name {
         args.extend(vec!["-l", login_name.as_ref()]);
      } if fullname.as_ref() != &self.fullname {
         args.extend(vec!["-c", fullname.as_ref()]);
      } if login_shell.as_ref() != &self.login_shell {
         args.extend(vec!["-s", login_shell.as_ref()]);
      } if let Some(home_dir) = &home_dir {
         args.extend(vec!["-m", "-d", home_dir.as_ref()]);
      } 

      if args.is_empty() {
         Ok(())
      } else {
         args.push(self.username());
         let _ = exec_cmd(USER_MOD, args)?;
         Ok(())
      }
   }

   pub fn change_password<T: AsRef<str>>(&mut self, curr_pwd: T, pwd: T, verify_pwd: T) -> Result<(), Error> {
      exec_cmd(PASSWD, vec![curr_pwd.as_ref(), pwd.as_ref(), verify_pwd.as_ref()])?;
      Ok(())
   }

   pub fn delete(&mut self) -> Result<(), Error> {
      let mut args = if std::path::PathBuf::from(&self.home_dir).exists() {
         vec!["-r"]
      } else {
         Vec::new()
      };
      args.push(self.username());
      exec_cmd(USER_DEL, args)?;
      Ok(())
   }

   pub fn groups(&self) -> Result<Vec<String>, Error> {
      let stdout = exec_cmd(GROUPS, vec![self.username()])?;
      Ok(get_list_by_sep(&stdout, " "))
   }

   pub fn username(&self) -> &String {
      &self.usrname
   }

   pub fn set_account_type(mut self, account_type: AccountType) -> Self {
      self.acc_type = account_type;
      self
   }
}