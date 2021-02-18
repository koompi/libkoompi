mod users;
mod groups;
mod account_type;

pub use users::*;
pub use groups::*;
pub use account_type::AccountType;
use std::io::Error;
use crate::helpers::{get_list_by_sep, exec_cmd, read_lines};

const GETENT: &str = "getent";
const CHSH: &str = "chsh";
pub(super) const PASSWD: &str = "passwd";
pub(super) const ADM_GROUP: &str = "wheel";
const GROUP: &str = "group";
const USERS_DB_PATH: &str = "/etc/passwd";
const GROUP_DB_PATH: &str = "/etc/group";
const MIN_UID: u16 = 1000;
const MAX_UID: u16 = 2000;

/// Structure of Users & Groups Manager
#[derive(Debug, Clone, Default)]
pub struct UsersGroupsManager {
   ls_users: Vec<User>,
   ls_groups: Vec<Group>,
}

// Public API
impl UsersGroupsManager {
   /// This method is used to initialize Users & Groups manager.
   pub fn new() -> Result<Self, Error> {
      let mut ug_mn = Self::default();
      ug_mn.load_users()?;
      ug_mn.load_groups()?;
      Ok(ug_mn)
   }

   /// This method is used to create a new user after check for username exists and then refresh users database.
   pub fn create_user<T: AsRef<str>>(&mut self, fullname: T, usrname: T, account_type: AccountType, pwd: T, verify_pwd: T) -> Result<bool, Error> {
      let mut res = false;
      if !self.ls_users.iter().any(|user| user.username() == usrname.as_ref()) {
         User::new(fullname, usrname, account_type, pwd, verify_pwd)?;
         self.load_users()?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to create a new group after check for group name exists and then refresh groups database.
   pub fn create_group<T: AsRef<str>>(&mut self, gname: T) -> Result<bool, Error> {
      let mut res = false;
      if !self.ls_groups.iter().any(|group| group.name() == gname.as_ref()) {
         Group::new(gname)?;
         self.load_groups()?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to list all available Login Shells.
   pub fn login_shells() -> Result<Vec<String>, Error> {
      let stdout = exec_cmd(CHSH, vec!["-l"])?;
      Ok(stdout.lines().map(ToString::to_string).collect())
   }
} 

// Private API
impl UsersGroupsManager {
   /// Refresh users database after any update
   fn load_users(&mut self) -> Result<(), Error> {
      // let users_stdout = exec_spawn_cmd(GETENT, vec![PASSWD, &format!("{{ {}..{} }}", MIN_UID, MAX_UID)])?;
      let allusers = read_lines(USERS_DB_PATH)?;
      let admin_members_stdout = exec_cmd(GETENT, vec![GROUP, ADM_GROUP])?;
      let admin_members = &get_list_by_sep(&admin_members_stdout, ":")[3];
      let ls_admin_usrnames = get_list_by_sep(&admin_members, ",");
      self.ls_users = allusers.map(|line| {
         if let Ok(line) = line {
            let ls_fields = get_list_by_sep(&line, ":");
            let uid: u16 = ls_fields[2].parse().unwrap();
            if MIN_UID < uid && uid < MAX_UID {
               Some(User::from_vec(ls_fields.as_ref(), ls_admin_usrnames.iter().map(AsRef::as_ref).collect()))
            } else {
               None
            }
         } else {
            None
         }
      })
      .filter_map(|u| u).collect();
      Ok(())
   }

   /// Refresh groups database after any update
   fn load_groups(&mut self) -> Result<(), Error> {
      // let groups_stdout = exec_spawn_cmd(GETENT, vec![GROUP, &format!("{{ {}..{} }}", MIN_UID, MAX_UID)])?;
      let allgroups = read_lines(GROUP_DB_PATH)?;
      self.ls_groups = allgroups.map(|line| {
         if let Ok(line) = line {
            let ls_fields = get_list_by_sep(&line, ":");
            let gid: u16 = ls_fields[2].parse().unwrap();
            if MIN_UID < gid && gid < MAX_UID {
               Some(Group::from_vec(ls_fields.as_ref()))
            } else {
               None
            }
         } else {
            None
         }
      })
      .filter_map(|u| u).collect();
      Ok(())
   }
}

#[cfg(test)]
mod test {
   use super::UsersGroupsManager;
   use std::io::Error;

   #[test]
   fn test_users_manager() -> Result<(), Error> {
      match UsersGroupsManager::new() {
         Ok(mut usr_mn) => {
            if usr_mn.create_group("test")? {
               println!("{:#?}", usr_mn);
            } else {
               println!("can not create user -- username is existing -- try again with new name");
            }
         },
         Err(err) => eprintln!("{:?}", err)
      }
      assert_eq!(true, false);
      Ok(())
   }
}