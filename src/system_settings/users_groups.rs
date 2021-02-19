mod users;
mod groups;
mod account_type;

pub use users::*;
pub use groups::*;
pub use account_type::AccountType;
use std::io::Error;
use crate::helpers::{get_list_by_sep, exec_cmd, read_lines};

pub(super) const PASSWD: &str = "passwd";
pub(super) const ADM_GROUP: &str = "wheel";
const GETENT: &str = "getent";
const CHSH: &str = "chsh";
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

   /// This method is used to change user account type by specified username and account type.
   pub fn change_user_type<T: AsRef<str>>(&mut self, usrname: T, account_type: AccountType) -> Result<bool, Error> {
      let mut res = false;
      if let Some(usr) = self.get_mut_user(usrname) {
         usr.change_account_type(account_type)?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to change user account information by specified username.
   pub fn change_user_info<T: AsRef<str>>(&mut self, usrname: T, uid: T, gname: T, fullname: T, login_name: T, login_shell: T, home_dir: T) -> Result<bool, Error> {
      let ls_users = self.ls_users.clone();
      let ls_groups = self.ls_groups.clone();
      if let Some(usr) = self.get_mut_user(usrname) {
         let usr_id = match uid.as_ref().to_string().parse::<u16>() {
            Ok(uid) => {
               if MIN_UID < uid && uid < MAX_UID && !ls_users.iter().any(|usr| usr.uid() == uid) {
                  Some(uid.to_string())
               } else {
                  None
               }
            },
            Err(_) => None
         };
         let grp_name = if !ls_groups.iter().any(|grp| grp.name() == gname.as_ref()) {Some(gname.as_ref().to_string())} else {None};
         let usrname = if !ls_users.iter().any(|usr| usr.username() == login_name.as_ref()) {Some(login_name.as_ref().to_string())} else {None};
         let home_dir = if !ls_users.iter().any(|usr| usr.home_dir() == home_dir.as_ref()) {Some(home_dir.as_ref().to_string())} else {None};
         usr.change_info(usr_id, grp_name, fullname.as_ref().to_string(), usrname, login_shell.as_ref().to_string(), home_dir)
      } else {
         Ok(false)
      }
   }

   /// This method is used to change user password by specified username.
   pub fn change_user_password<T: AsRef<str>>(&mut self, usrname: T, curr_pwd: T, pwd: T, verify_pwd: T) -> Result<bool, Error> {
      let mut res = false;
      if let Some(usr) = self.get_mut_user(usrname) {
         usr.change_password(curr_pwd.as_ref(), pwd.as_ref(), verify_pwd.as_ref())?;
         res = true;
      }
      Ok(res)
   }

   pub fn reset_user_password<T: AsRef<str>>(&mut self, usrname: T, pwd: T, verify_pwd: T) -> Result<bool, Error> {
      let mut res = false;
      if let Some(_) = self.get_mut_user(usrname.as_ref()) {
         User::reset_password(usrname.as_ref(), pwd.as_ref(), verify_pwd.as_ref())?;
         res = true;
      }
      Ok(res)
   }

   pub fn delete_user<T: AsRef<str>>(&mut self, usrname: T) -> Result<bool, Error> {
      let mut res = false;
      if let Some(usr) = self.get_mut_user(usrname) {
         usr.delete()?;
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

   /// This method is used to change group name by specified current group name and new group name.
   pub fn change_group_name<T: AsRef<str>>(&mut self, gname: T, new_name: T) -> Result<bool, Error> {
      if let Some(group) = self.get_mut_group(gname){
         group.change_name(new_name)
      } else {
         Ok(false)
      }
   }

   /// This method is used to set/change list of members of the group by specified group name.
   pub fn change_group_members<T: AsRef<str>>(&mut self, gname: T, ls_members: Vec<&str>) -> Result<bool, Error> {
      let mut res = false;
      if let Some(group) = self.get_mut_group(gname) {
         group.change_membership(ls_members)?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to delete group by specified group name.
   pub fn delete_group<T: AsRef<str>>(&mut self, gname: T) -> Result<bool, Error> {
      let mut res = false;
      if let Some(group) = self.get_mut_group(gname) {
         group.delete()?;
         self.load_groups()?;
         res = true;
      } 
      Ok(res)
   }

   /// This method is used to get current list of users account.
   pub fn list_users(&self) -> &[User] {
      self.ls_users.as_slice()
   }

   /// This method is used to get current list of groups account.
   pub fn list_groups(&self) -> &[Group] {
      self.ls_groups.as_slice()
   }

   /// This method is used to list all available Login Shells.
   pub fn login_shells() -> Result<Vec<String>, Error> {
      let stdout = exec_cmd(CHSH, vec!["-l"])?;
      Ok(stdout.lines().map(ToString::to_string).collect())
   }
} 

// Private API
impl UsersGroupsManager {
   /// Refresh users database after any update.
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

   /// Refresh groups database after any update.
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

   fn get_mut_group<T: AsRef<str>>(&mut self, gname: T) -> Option<&mut Group> {
      self.ls_groups.iter_mut().find(|g| g.name() == gname.as_ref()) 
   }

   fn get_mut_user<T: AsRef<str>>(&mut self, usrname: T) -> Option<&mut User> {
      self.ls_users.iter_mut().find(|usr| usr.username() == usrname.as_ref())
   }
}

#[cfg(test)]
mod test {
   use super::{UsersGroupsManager, AccountType};
   use std::io::Error;

   #[test]
   fn test_users_manager() -> Result<(), Error> {
      match UsersGroupsManager::new() {
         Ok(mut usr_mn) => {
            // println!("{:#?}", usr_mn);
            if usr_mn.create_group("test")? {
               println!("successfully create test group");
               if usr_mn.create_user("Test User", "test", AccountType::Normal, "123", "123")? {
                  println!("successfully create test user");
                  // if usr_mn.delete_user("test")? {
                  //    println!("successfully delete test user");
                  //    if usr_mn.delete_group("test")? {
                  //       println!("successfully delete test group");
                  //    } else {
                  //       println!("can not delete group -- group name is not existing -- try again with new name");
                  //    }
                  // } else { 
                  //    println!("can not delete user -- user name is not existing -- try again with new name");
                  // }
               } else { 
                  println!("can not create user -- user name is existing -- try again with new name");
               }
            } else {
               println!("can not create group -- group name is existing -- try again with new name");
            }
         },
         Err(err) => eprintln!("{:?}", err)
      }
      assert_eq!(true, false);
      Ok(())
   }
}