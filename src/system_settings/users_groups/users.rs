use std::fmt::Display;
use std::io::Error;
use super::groups::GPASSWD;
use super::account_type::AccountType;
use super::super::users_groups::{ADM_GROUP, PASSWD};
use crate::helpers::{exec_cmd, get_list_by_sep, constants::PKEXEC};

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
   groups: Vec<String>,
}

impl User {
   pub(super) fn new<T: AsRef<str>>(fullname: T, usrname: T, account_type: AccountType, pwd: T, verify_pwd: T) -> Result<(), Error> {
      exec_cmd(PKEXEC, vec![USER_ADD, "-c", fullname.as_ref(), "-N", usrname.as_ref()])?;
      if account_type == AccountType::Admin {
         exec_cmd(GPASSWD, vec!["-a", usrname.as_ref(), ADM_GROUP])?;
      }
      User::reset_password(usrname, pwd, verify_pwd)
   }

   pub(super) fn from_vec<T: AsRef<str> + Display>(fields: &[T], ls_admin: Vec<&str>) -> Self {
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
         acc_type: if ls_admin.contains(&usrname.as_str()) {AccountType::Admin} else {AccountType::Normal},
         ..Self::default()
      }
   }

   pub fn change_account_type(&mut self, account_type: AccountType) -> Result<String, Error> {
      let opt = match account_type {
         AccountType::Normal => "-d",
         AccountType::Admin => "-a",
      };
      let msg = exec_cmd(GPASSWD, vec![opt, self.username(), ADM_GROUP])?;
      self.acc_type = account_type;
      Ok(msg)
   }

   pub fn change_info<T: AsRef<str>>(&mut self, uid: Option<T>, gname: Option<T>, fullname: T, login_name: Option<T>, login_shell: T, home_dir: Option<T>) -> Result<(), Error> {
      let mut args = Vec::new();
      if let Some(uid) = &uid {
         args.extend(vec!["-u", uid.as_ref()]);
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
         exec_cmd(USER_MOD, args)?;
         if let Some(uid) = uid {
            self.uid = uid.as_ref().to_string().parse().unwrap();
         } if let Some(login_name) = login_name {
            self.usrname = login_name.as_ref().to_string();
         }
         self.fullname = fullname.as_ref().to_string();
         self.login_shell = login_shell.as_ref().to_string();
         if let Some(home_dir) = home_dir {
            self.home_dir = home_dir.as_ref().to_string();
         }
         Ok(())
      }
   }

   pub fn change_password<T: AsRef<str>>(&mut self, curr_pwd: T, pwd: T, verify_pwd: T) -> Result<(), Error> {
      exec_cmd(PASSWD, vec![curr_pwd.as_ref(), pwd.as_ref(), verify_pwd.as_ref()])?;
      Ok(())
   }

   pub fn can_reset_password(&mut self) -> bool {
      self.acc_type == AccountType::Admin
   }

   pub fn reset_password<T: AsRef<str>>(usrname: T, pwd: T, verify_pwd: T) -> Result<(), Error> {
      exec_cmd(PASSWD, vec![usrname.as_ref(), pwd.as_ref(), verify_pwd.as_ref()])?;
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

   pub fn fetch_groups(&mut self) -> Result<(), Error> {
      let stdout = exec_cmd(GROUPS, vec![self.username()])?;
      self.groups = get_list_by_sep(&stdout, " ");
      Ok(())
   }

   pub fn set_account_type(mut self, account_type: AccountType) -> Self {
      self.acc_type = account_type;
      self
   }

   pub fn uid(&self) -> u16 {
      self.uid
   }

   pub fn gid(&self) -> u16 {
      self.gid
   }

   pub fn username(&self) -> &String {
      &self.usrname
   }

   pub fn account_type(&self) -> AccountType {
      self.acc_type
   }

   pub fn fullname(&self) -> &String {
      &self.fullname
   }

   pub fn login_shell(&self) -> &String {
      &self.login_shell
   }

   pub fn home_dir(&self) -> &String {
      &self.home_dir
   }

   pub fn groups(&mut self) -> &[String] {
      self.groups.as_slice()
   }
}