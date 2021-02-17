mod users;
mod groups;

pub use users::*;
pub use groups::*;

use std::io::Error;
use crate::helpers::{get_list_by_sep, exec_pipe_cmd, exec_spawn_cmd, exec_cmd};

const GETENT: &str = "getent";
const AWK: &str = "awk";
const CHSH: &str = "chsh";
pub(super) const PASSWD: &str = "passwd";
pub(super) const GPASSWD: &str = "gpasswd";
pub(super) const ADM_GROUP: &str = "wheel";
const GROUP: &str = "group";
const MIN_UID: u16 = 1000;
const MAX_UID: u16 = 2000;

#[derive(Debug, Clone, Default)]
pub struct UsersGroupsManager {
   ls_users: Vec<User>,
}

impl UsersGroupsManager {
   pub fn new() -> Result<Self, Error> {
      let mut usr_mn = Self::default();
      let stdout = exec_spawn_cmd(GETENT, vec![PASSWD, &format!("{{ {}..{} }}", MIN_UID, MAX_UID)])?;
      let wheel_members_stdout = exec_pipe_cmd((GETENT, vec![GROUP, "wheel"]), (AWK, vec!["-F:", "\'{ print $4 }\'"]))?;
      usr_mn.ls_users = stdout.lines()
         .map(|line| User::from_vec(get_list_by_sep(line, ":").as_ref()))
         .map(|user| {
            if get_list_by_sep(&wheel_members_stdout, ",").contains(user.username()) {
               user.set_account_type(AccountType::Admin)
            } else {
               user.set_account_type(AccountType::Normal)
            }
         }).collect();
      Ok(usr_mn)
   }

   pub fn create_user<T: AsRef<str>>(fullname: T, usrname: T, account_type: AccountType, pwd: T, verify_pwd: T) -> Result<(), Error> {
      User::new(fullname, usrname, account_type, pwd, verify_pwd)
   }

   pub fn login_shells() -> Result<Vec<String>, Error> {
      let stdout = exec_cmd(CHSH, vec!["-l"])?;
      Ok(stdout.lines().map(ToString::to_string).collect::<Vec<String>>())
   }
} 

#[cfg(test)]
mod test {
   use super::UsersGroupsManager;

   #[test]
   fn test_users_manager() {
      match UsersGroupsManager::new() {
         Ok(usr_mn) => println!("{:#?}", usr_mn),
         Err(err) => eprintln!("{:?}", err)
      }
      assert_eq!(true, false);
   }
}