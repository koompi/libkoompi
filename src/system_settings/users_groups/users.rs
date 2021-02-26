use std::fmt::Display;
use std::io::Error;
use std::path::{Path, PathBuf};
use super::groups::GPASSWD;
use super::account_type::AccountType;
use super::super::users_groups::{ADM_GROUP, PASSWD};
use crate::helpers::{exec_cmd, exec_spawn_cmd, get_list_by_sep, constants::PKEXEC, to_formatted_name};

const USER_ADD: &str = "useradd";
const USER_MOD: &str = "usermod";
const USER_DEL: &str = "userdel";
const GROUPS: &str = "groups";
const PROFILE_ROOT_PATH: &str = "/var/lib/AccountsService/icons";

/// Structure of User Account
#[derive(Debug, Clone, Default)]
pub struct User {
   uid: u16,
   gid: u16,
   acc_type: AccountType,
   fullname: String,
   usrname: String,
   login_shell: PathBuf,
   home_dir: PathBuf,
   groups: Vec<String>,
}

// Public API
impl User {
   /// This method is used to create a new user without creating personal group and add to ADMIN group if account type is admin.
   pub(super) fn new<T: AsRef<str>>(fullname: T, usrname: T, account_type: AccountType, pwd: T, verify_pwd: T) -> Result<(), Error> {
      let args = vec![USER_ADD, "-c", fullname.as_ref(), "-m", "-N", usrname.as_ref()];
      exec_cmd(PKEXEC, args)?;
      let mut gpasswd_args = vec![USER_MOD, "-a", "-G"];
      let mut groups = vec!["input", "cups"];
      if account_type == AccountType::Admin {
         groups.push(ADM_GROUP);
      }
      let concat_groups = groups.join(",");
      gpasswd_args.extend(vec![concat_groups.as_str(), usrname.as_ref()]);
      exec_cmd(PKEXEC, gpasswd_args)?;
      User::reset_password(usrname, pwd, verify_pwd)
   }

   /// This method is used to create a user entry object from a list of fields and list of admin usernames.
   pub(super) fn from_vec<T: AsRef<str> + Display>(fields: &[T], ls_admin: Vec<&str>) -> Self {
      let mut iter = fields.iter();
      let usrname = iter.next().unwrap().to_string();
      iter.next().unwrap();
      let uid: u16 = iter.next().unwrap().to_string().parse().unwrap();
      let gid: u16 = iter.next().unwrap().to_string().parse().unwrap();
      let fullname = iter.next().unwrap();
      let home_dir = iter.next().unwrap();
      let login_shell = iter.next().unwrap();
      let groups = match exec_cmd(GROUPS, vec![usrname.as_ref()]) {
         Ok(output) => get_list_by_sep(&output, " "),
         Err(_) => Vec::new()
      };

      Self {
         uid, gid, groups,
         fullname: fullname.to_string(), 
         usrname: usrname.clone(), 
         login_shell: PathBuf::from(&login_shell.to_string()), 
         home_dir: PathBuf::from(&home_dir.to_string()), 
         acc_type: if ls_admin.contains(&usrname.as_str()) {AccountType::Admin} else {AccountType::User},
      }
   }

   /// This method is used to toggle account type of the user and return a message. 
   pub(super) fn change_account_type(&mut self, account_type: AccountType) -> Result<(), Error> {
      let opt = match account_type {
         AccountType::User => "-d",
         AccountType::Admin => "-a",
      };
      exec_cmd(PKEXEC, vec![GPASSWD, opt, self.username(), ADM_GROUP])?;
      self.acc_type = account_type;
      Ok(())
   }

   /// This method is used to change user account information except account typpe and password.
   pub(super) fn change_info<T: AsRef<str>, P: AsRef<Path>>(&mut self, uid: Option<T>, gid: Option<T>, fullname: T, login_name: Option<T>, login_shell: Option<P>, home_dir: Option<P>) -> Result<bool, Error> {
      let mut args = Vec::new();
      let usrname = self.usrname.clone();
      if let Some(uid) = &uid {
         if let Ok(uid_u16) = uid.as_ref().to_string().parse() { 
            args.extend(vec!["-u", uid.as_ref()]);
            self.uid = uid_u16;
         } 
      } if let Some(gid) = &gid {
         args.extend(vec!["-g", gid.as_ref()]);
         if let Ok(gid) = gid.as_ref().to_string().parse() {
            self.gid = gid;
         }
      } if fullname.as_ref().ne(self.fullname().as_str()) {
         args.extend(vec!["-c", fullname.as_ref()]);
         self.fullname = fullname.as_ref().to_string();
      } if let Some(login_name) = &login_name {
         args.extend(vec!["-l", login_name.as_ref()]);
         self.usrname = login_name.as_ref().to_string();
      } if let Some(login_shell) = &login_shell {
         if login_shell.as_ref().ne(&self.login_shell) {
            args.extend(vec!["-s", login_shell.as_ref().to_str().unwrap()]);
            self.login_shell = login_shell.as_ref().into();
         } 
      } if let Some(home_dir) = &home_dir {
         args.extend(vec!["-m", "-d", home_dir.as_ref().to_str().unwrap()]);
         self.home_dir = home_dir.as_ref().into();
      }

      if args.is_empty() {
         Ok(false)
      } else {
         args.insert(0, USER_MOD);
         args.push(usrname.as_str());
         exec_cmd(PKEXEC, args)?;
         Ok(true)
      }
   }

   /// This method is used to change password for the user account.
   pub(super) fn change_password<T: AsRef<str>>(&mut self, curr_pwd: T, pwd: T, verify_pwd: T) -> Result<(), Error> {
      exec_spawn_cmd(PASSWD, Vec::new(), Some(&vec![curr_pwd.as_ref(), pwd.as_ref(), verify_pwd.as_ref()].join("\n")))?;
      Ok(())
   }

   /// This method is used to change secondary groups.
   pub(super) fn change_groups(&mut self, ls_grps: Vec<&str>) -> Result<(), Error> {
      let grps_str = ls_grps.join(",");
      exec_cmd(PKEXEC, vec![USER_MOD, "-a", "-G", &grps_str, &self.usrname])?;
      self.groups = ls_grps.into_iter().map(ToOwned::to_owned).collect();
      Ok(())
   }

   /// This method is used to check whether this user has permission to reset other users account's password.
   pub fn is_admin(&self) -> bool {
      self.acc_type == AccountType::Admin
   }

   /// This method is used to reset other users account's password.
   pub(super) fn reset_password<T: AsRef<str>>(usrname: T, pwd: T, verify_pwd: T) -> Result<(), Error> {
      exec_spawn_cmd(PKEXEC, vec![PASSWD, usrname.as_ref()], Some(&vec![pwd.as_ref(), verify_pwd.as_ref()].join("\n")))?;
      Ok(())
   }

   /// This method is used to delete this user account from database.
   pub(super) fn delete(&mut self, delete_home_dir: bool) -> Result<(), Error> {
      let mut args = vec![USER_DEL];
      if delete_home_dir {
         if std::path::PathBuf::from(&self.home_dir).exists() {
            args.push("-r");
         }
      }
      args.push(&self.usrname);
      exec_cmd(PKEXEC, args)?;
      Ok(())
   }

   /// This method is return UID.
   pub fn uid(&self) -> u16 {
      self.uid
   }

   /// This method is return GID (Primary GID).
   pub fn gid(&self) -> u16 {
      self.gid
   }

   /// This method is return User name/Login name.
   pub fn username(&self) -> &String {
      &self.usrname
   }

   /// This method is return Account Type.
   pub fn account_type(&self) -> AccountType {
      self.acc_type
   }

   /// This method is return Fullname or Username if fullname not exist.
   pub fn fullname(&self) -> String {
      let name = if self.fullname.is_empty() {
         &self.usrname
      } else {
         &self.fullname
      };
      to_formatted_name(name)
   }

   /// This method is return Login Shell.
   pub fn login_shell(&self) -> &PathBuf {
      &self.login_shell
   }

   /// This method is return Home Directory.
   pub fn home_dir(&self) -> &PathBuf {
      &self.home_dir
   }

   /// This method is return List of group name. Note: You need to call fetch_groups method first.
   pub fn groups(&self) -> &[String] {
      self.groups.as_slice()
   }

   /// This method is return profile picture path.
   pub fn profile_path(&self) -> PathBuf {
      PathBuf::from(PROFILE_ROOT_PATH).join(&self.usrname)
   }
}