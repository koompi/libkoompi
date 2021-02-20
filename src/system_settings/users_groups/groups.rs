use std::fmt::Display;
use std::io::Error;
use crate::helpers::{exec_cmd, get_list_by_sep, constants::PKEXEC};

pub(super) const GPASSWD: &str = "gpasswd";
const GROUP_ADD: &str = "groupadd";
const GROUP_MOD: &str = "groupmod";
const GROUP_DEL: &str = "groupdel";

/// Structure of Group Account
#[derive(Debug, Clone, Default)]
pub struct Group {
   gid: u16,
   gname: String,
   members: Vec<String>
}

// Public API
impl Group {
   /// This method is used to create a new group with group name.
   pub(super) fn new<T: AsRef<str>>(gname: T) -> Result<(), Error> {
      exec_cmd(PKEXEC, vec![GROUP_ADD, gname.as_ref()])?;
      Ok(())
   }

   /// This method is used to create a group entry object from a list of fields.
   pub(super) fn from_vec<T: AsRef<str> + Display>(fields: &[T]) -> Self {
      let mut iter = fields.iter();
      let gname = iter.next().unwrap();
      iter.next().unwrap();
      let gid: u16 = iter.next().unwrap().to_string().parse().unwrap();
      let members = get_list_by_sep(iter.next().unwrap().to_string().as_str(), ",");

      Self {
         gid, 
         gname: gname.to_string(), 
         members
      }
   }

   /// This method is used to set/change list of members of the group instance.
   pub fn change_membership(&mut self, ls_members: Vec<&str>) -> Result<(), Error> {
      if ls_members != self.members.iter().map(AsRef::as_ref).collect::<Vec<&str>>() {
         let formatted_members = ls_members.join(",");
         exec_cmd(PKEXEC, vec![GPASSWD, "-M", &formatted_members, &self.gname])?;
         self.members = ls_members.iter().map(ToString::to_string).collect();
      }
      Ok(())
   }

   /// This method is used to add new user to the group after check current members exists.
   pub fn add_user<T: AsRef<str>>(&mut self, usrname: T) -> Result<bool, Error> {
      let mut res = false;
      if !self.members.contains(&usrname.as_ref().to_string()) {
         exec_cmd(PKEXEC, vec![GPASSWD, "-a", usrname.as_ref(), &self.gname])?;
         self.members.push(usrname.as_ref().to_string());
         res = true;
      }
      Ok(res)
   }

   /// This method is used to remove user from the group after check current members exists.
   pub fn remove_user<T: AsRef<str>>(&mut self, usrname: T) -> Result<bool, Error> {
      let mut res = false;
      if self.members.contains(&usrname.as_ref().to_string()) {
         exec_cmd(PKEXEC, vec![GPASSWD, "-d", usrname.as_ref(), &self.gname])?;
         let idx = self.members.iter().position(|m| m == &usrname.as_ref().to_string());
         if let Some(idx) = idx {
            self.members.remove(idx);
            res = true;
         }
      }
      Ok(res)
   }

   /// This method is used to append users to the group after dubplicated members. 
   pub fn append_users(&mut self, ls_members: Vec<&str>) -> Result<(), Error> {
      let mut all_members: Vec<&str> = self.members.iter().map(AsRef::as_ref).chain(ls_members).collect();
      all_members.sort();
      all_members.dedup();
      let formatted_members = all_members.join(",");
      exec_cmd(PKEXEC, vec![GPASSWD, "-M", &formatted_members, &self.gname])?;
      self.members = all_members.iter().map(ToString::to_string).collect();
      Ok(())
   }

   /// This method is used to change group name after check with current name.
   pub fn change_name<T: AsRef<str>>(&mut self, new_gname: T) -> Result<bool, Error> {
      let mut res = false;
      if new_gname.as_ref() != self.gname.as_str() {
         exec_cmd(PKEXEC, vec![GROUP_MOD, "-n", new_gname.as_ref(), self.gname.as_str()])?;
         self.gname = new_gname.as_ref().to_string();
         res = true;
      } 
      Ok(res)
   }

   /// This method is used to delete the group from database.
   pub(super) fn delete(&mut self) -> Result<(), Error> {
      exec_cmd(PKEXEC, vec![GROUP_DEL, &self.gname])?;
      Ok(())
   }

   /// This method is return group GID.
   pub fn gid(&self) -> u16 {
      self.gid
   }

   /// This method is return group Name.
   pub fn name(&self) -> &String {
      &self.gname
   }

   /// This method is return group members.
   pub fn members(&self) -> &[String] {
      self.members.as_slice()
   }
}