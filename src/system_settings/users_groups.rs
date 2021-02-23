mod users;
mod groups;
mod account_type;

pub use users::User;
pub use groups::Group;
pub use account_type::AccountType;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::collections::HashSet;
use crate::helpers::{get_list_by_sep, exec_cmd, read_lines, to_account_name};

const PASSWD: &str = "passwd";
const GREP: &str = "grep";
const CHSH: &str = "chsh";
const ID: &str = "id";
const ADM_GROUP: &str = "wheel";
const USERS_DB_PATH: &str = "/etc/passwd";
const GROUP_DB_PATH: &str = "/etc/group";
const MIN_UID: u16 = 1000;
const MAX_UID: u16 = 2000;

/// Structure of Users & Groups Manager
#[derive(Debug, Clone, Default)]
pub struct UsersGroupsManager {
   curr_uid: u16,
   ls_users: Vec<User>,
   ls_groups: Vec<Group>,
   login_shells: Vec<String>,
}

// Public API
impl UsersGroupsManager {
   /// This method is used to initialize Users & Groups manager.
   pub fn new() -> Result<Self, Error> {
      let mut ug_mn = Self::default();
      ug_mn.load_users()?;
      ug_mn.load_groups()?;
      ug_mn.load_curr_user()?;
      ug_mn.load_login_shells()?;
      Ok(ug_mn)
   }

   /// This method is used to create a new user after check for username exists and then refresh users database.
   pub fn create_user<T: AsRef<str> + Clone>(&mut self, fullname: T, usrname: T, account_type: AccountType, pwd: T, verify_pwd: T) -> Result<bool, Error> {
      let mut res = false;
      let usrname = to_account_name(usrname);
      if !self.ls_users.iter().any(|user| user.username().eq(&usrname)) {
         User::new(fullname.as_ref(), usrname.as_str(), account_type, pwd.as_ref(), verify_pwd.as_ref())?;
         self.load_users()?;
         res = true;
      } 
      Ok(res)
   }

   // /// This method is used to change user account type by specified username and account type.
   // pub fn change_user_type<T: AsRef<str>>(&mut self, usrname: T, account_type: AccountType) -> Result<bool, Error> {
   //    let mut res = false;
   //    if let Some(usr) = self.get_mut_user(usrname) {
   //       usr.change_account_type(account_type)?;
   //       res = true;
   //    }
   //    Ok(res)
   // }

   /// This method is used to change user account information by specified username.
   pub fn change_user_info<T: AsRef<str>, P: AsRef<Path>>(&mut self, usrname: T, uid: Option<T>, gname: T, fullname: T, login_name: Option<T>, login_shell: P, home_dir: Option<P>) -> Result<bool, Error> {
      let ls_users = self.ls_users.clone();
      let ls_groups = self.ls_groups.clone();
      let login_shells = self.login_shells.clone();
      let usrname = to_account_name(usrname);
      let gname = to_account_name(gname);
      if let Some(usr) = self.get_mut_user(&usrname) {
         let usr_id = match uid {
            Some(uid) => match uid.as_ref().to_string().parse::<u16>() {
               Ok(uid) => {
                  if MIN_UID < uid && uid < MAX_UID && !ls_users.iter().any(|usr| usr.uid().eq(&uid)) {
                     Some(uid.to_string())
                  } else {
                     None
                  }
               },
               Err(_) => None
            },
            None => None
         };
         let grp_name = match gname.parse::<u16>() {
            Ok(gid) => {
               if MIN_UID < gid && gid < MAX_UID && ls_groups.iter().any(|grp| grp.gid().eq(&gid)) {
                  Some(gid.to_string())
               } else {
                  None
               } 
            },
            Err(_) => {
               if ls_groups.iter().any(|grp| grp.name().eq(&gname)) {
                  Some(gname)
               } else {
                  None
               }
            }
         };
         let usrname = match login_name {
            Some(login_name) => {
               let login_name = to_account_name(login_name);
               if !ls_users.iter().any(|usr| usr.username().eq(&login_name)) {Some(login_name)} else {None}
            },
            None => None
         };
         let home_dir = match home_dir {
            Some(home_dir) => if !ls_users.iter().any(|usr| usr.home_dir().eq(home_dir.as_ref())) {Some(home_dir)} else {None},
            None => None
         };
         let login_shell = if login_shells.iter().any(|sh| login_shell.as_ref().eq(Path::new(sh))) {Some(login_shell)} else {None};
         usr.change_info(usr_id, grp_name, fullname.as_ref().to_string(), usrname, login_shell, home_dir)
      } else {
         Ok(false)
      }
   }

   /// This method is used to change user password by specified username.
   pub fn change_user_password<T: AsRef<str>>(&mut self, usrname: T, curr_pwd: T, pwd: T, verify_pwd: T) -> Result<bool, Error> {
      let mut res = false;
      let usrname = to_account_name(usrname);
      if let Some(usr) = self.get_mut_user(&usrname) {
         usr.change_password(curr_pwd.as_ref(), pwd.as_ref(), verify_pwd.as_ref())?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to reset user password by specified username and password.
   pub fn reset_user_password<T: AsRef<str>>(&mut self, usrname: T, pwd: T, verify_pwd: T) -> Result<bool, Error> {
      let mut res = false;
      let usrname = to_account_name(usrname);
      if let Some(_) = self.get_mut_user(&usrname) {
         User::reset_password(usrname.as_str(), pwd.as_ref(), verify_pwd.as_ref())?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to return list of groups belongs to the user by specified username, or return None if not found.
   pub fn user_groups<T: AsRef<str>>(&self, usrname: T) -> Option<Vec<&Group>> {
      let usrname = to_account_name(usrname);
      if let Some(usr) = self.get_user(&usrname) {
         Some(self.list_groups().iter().filter(|grp| usr.groups().contains(grp.name())).map(ToOwned::to_owned).collect())
      } else {
         None
      }
   }

   /// This method is used to delete a user from database by specified username.
   pub fn delete_user<T: AsRef<str>>(&mut self, usrname: T, delete_home_dir: bool) -> Result<bool, Error> {
      let usrname = to_account_name(usrname);
      let mut res = false;
      if let Some(usr) = self.get_mut_user(&usrname) {
         usr.delete(delete_home_dir)?;
         self.load_users()?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to create a new group after check for group name exists and then refresh groups database.
   pub fn create_group<T: AsRef<str> + Clone>(&mut self, gname: T) -> Result<bool, Error> {
      let mut res = false;
      let gname = to_account_name(gname);
      if !self.ls_groups.iter().any(|group| group.name().eq(&gname)) {
         Group::new(gname)?;
         self.load_groups()?;
         res = true;
      } 
      Ok(res)
   }

   /// This method is used to change group name by specified current group name and new group name.
   pub fn change_group_name<T: AsRef<str>>(&mut self, gname: T, new_name: T) -> Result<bool, Error> {
      let gname = to_account_name(gname);
      let new_name = to_account_name(new_name);
      let ls_groups = self.ls_groups.clone();
      if let Some(group) = self.get_mut_group(&gname){
        if ls_groups.iter().any(|grp| grp.name().eq(&new_name)) {
            group.change_name(new_name)
         } else {
            Ok(false)
         }
      } else {
         Ok(false)
      }
   }

   /// This method is used to set/change list of members of the group by specified group name.
   pub fn change_group_members<T: AsRef<str>>(&mut self, gname: T, ls_members: Vec<&str>) -> Result<bool, Error> {
      let mut res = false;
      let gname = to_account_name(gname);
      if let Some(group) = self.get_mut_group(&gname) {
         group.change_membership(ls_members)?;
         res = true;
      }
      Ok(res)
   }

   /// This method is used to return a list of users of a group by specified group name, or None if not found.
   pub fn group_members<T: AsRef<str>>(&self, gname: T) -> Option<Vec<&User>> {
      let gname = to_account_name(gname);
      if let Some(group) = self.get_group(&gname) {
         Some(self.list_users().iter().filter(|&usr| group.members().contains(usr.username())).map(ToOwned::to_owned).collect())
      } else {
         None
      }
   }

   /// This method is used to delete group by specified group name.
   pub fn delete_group<T: AsRef<str>>(&mut self, gname: T) -> Result<bool, Error> {
      let mut res = false;
      let gname = to_account_name(gname);
      if let Some(group) = self.get_mut_group(&gname) {
         group.delete()?;
         self.load_groups()?;
         res = true;
      } 
      Ok(res)
   }

   /// This method is used to return current running user UID.
   pub fn current_uid(&self) -> u16 {
      self.curr_uid
   }

   /// This method is used to return all user accounts and system accounts available on system.
   pub fn all_users(&self) -> &[User] {
      self.ls_users.as_slice()
   }

   /// This method is used to get current list of users account.
   pub fn list_users(&self) -> Vec<&User> {
      let mut ls_users: Vec<&User> = self.ls_users.iter().filter(|usr| MIN_UID < usr.uid() && usr.uid() < MAX_UID).collect();
      if let Some(idx) = ls_users.iter().position(|usr| usr.uid().eq(&self.curr_uid)) {
         if idx != 0 {
            ls_users.swap(0, idx);
         }
      }
      ls_users
   }

   /// This method is used to return all user-defined group accounts and system accounts available on system.
   pub fn all_groups(&self) -> &[Group] {
      self.ls_groups.as_slice()
   }

   /// This method is used to get current list of groups account.
   pub fn list_groups(&self) -> Vec<&Group> {
      let ls_users_gid: HashSet<u16> = self.list_users().iter().map(|usr| usr.gid()).collect();
      self.ls_groups.iter().filter(|grp| MIN_UID < grp.gid() && grp.gid() < MAX_UID).filter(|grp| !ls_users_gid.contains(&grp.gid())).collect()
   }

   /// This method is used to return an User instance if given username is existing in database and user-defined user account.
   pub fn user_from_name<T: AsRef<str>>(&self, usrname: T) -> Option<&User> {
      self.list_users().iter().find(|usr| usr.username().eq(usrname.as_ref())).map(ToOwned::to_owned)
   }

   /// This method is used to return an Group instance if given group name is existing in database and user-defined group account.
   pub fn group_from_name<T: AsRef<str>>(&self, grpname: T) -> Option<&Group> {
      self.list_groups().iter().find(|grp| grp.name().eq(grpname.as_ref())).map(ToOwned::to_owned)
   }

   /// This method is used to list all available Login Shells.
   pub fn login_shells(&self) -> &[String] {
      self.login_shells.as_slice()
   }
} 

// Private API
impl UsersGroupsManager {
   /// Refresh users database after any update.
   fn load_users(&mut self) -> Result<(), Error> {
      let allusers = read_lines(USERS_DB_PATH)?;
      let admin_members_stdout = exec_cmd(GREP, vec![&format!("{}:", ADM_GROUP), GROUP_DB_PATH])?;
      let admin_members = &get_list_by_sep(&admin_members_stdout, ":")[3];
      let ls_admin_usrnames = get_list_by_sep(&admin_members, ",");
      self.ls_users = allusers.map(|line| if let Ok(line) = line {
         Some(User::from_vec(get_list_by_sep(&line, ":").as_ref(), ls_admin_usrnames.iter().map(AsRef::as_ref).collect()))
      } else {
         None
      }).filter_map(|usr| usr).collect();
      Ok(())
   }

   /// Refresh groups database after any update.
   fn load_groups(&mut self) -> Result<(), Error> {
      let allgroups = read_lines(GROUP_DB_PATH)?;
      self.ls_groups = allgroups.map(|line| if let Ok(line) = line {
         Some(Group::from_vec(get_list_by_sep(&line, ":").as_ref()))
      } else {
         None
      }).filter_map(|grp| grp).collect();
      Ok(())
   }

   /// Load current running user account.
   fn load_curr_user(&mut self) -> Result<(), Error> {
      let uid = exec_cmd(ID, vec!["-u"])?;
      match uid.parse::<u16>() {
         Ok(uid) => {
            self.curr_uid = uid;
            Ok(())
         },
         Err(err) => Err(Error::new(ErrorKind::Other, err))
      }
   }

   /// Load all available login shells.
   fn load_login_shells(&mut self) -> Result<(), Error> {
      let stdout = exec_cmd(CHSH, vec!["-l"])?;
      self.login_shells = stdout.lines().map(ToString::to_string).collect();
      Ok(())
   }

   fn get_mut_group<T: AsRef<str>>(&mut self, gname: T) -> Option<&mut Group> {
      self.ls_groups.iter_mut().find(|g| g.name().eq(gname.as_ref())) 
   }

   fn get_mut_user<T: AsRef<str>>(&mut self, usrname: T) -> Option<&mut User> {
      self.ls_users.iter_mut().find(|usr| usr.username().eq(usrname.as_ref()))
   }

   fn get_user<T: AsRef<str>>(&self, usrname: T) -> Option<&User>{
      self.ls_users.iter().find(|usr| usr.username().eq(usrname.as_ref()))
   }

   fn get_group<T: AsRef<str>>(&self, gname: T) -> Option<&Group> {
      self.ls_groups.iter().find(|g| g.name().eq(gname.as_ref())) 
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
            if usr_mn.create_group("test")? {
               println!("successfully create test group");
               if usr_mn.create_user("Test User", "test", AccountType::default(), "123", "123")? {
                  println!("successfully create test user");
                  if usr_mn.change_user_info("test", None, "users", "User Test", None, "/bin/fish", None)? {
                     println!("change info success");
                  } else {
                     println!("can not change info");
                  }
                  if usr_mn.delete_user("test", false)? {
                     println!("successfully delete test user");
                     if usr_mn.delete_group("test")? {
                        println!("successfully delete test group");
                     } else {
                        println!("can not delete group -- group name is not existing -- try again with new name");
                     }
                  } else { 
                     println!("can not delete user -- user name is not existing -- try again with new name");
                  }
               } else { 
                  println!("can not create user -- user name is existing -- try again with new name");
               }
            } else {
               println!("can not create group -- group name is existing -- try again with new name");
            }
            println!("{:#?}", usr_mn.list_groups()); 
         },
         Err(err) => eprintln!("{:?}", err)
      }
      assert_eq!(true, false);
      Ok(())
   }
}