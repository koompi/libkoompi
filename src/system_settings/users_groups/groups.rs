use std::fmt::Display;
use std::io::Error;
use crate::helpers::{exec_cmd, get_list_by_sep, constants::PKEXEC};

pub(super) const GPASSWD: &str = "gpasswd";
const GROUP_ADD: &str = "groupadd";
const GROUP_MOD: &str = "groupmod";
const GROUP_DEL: &str = "groupdel";

#[derive(Debug, Clone, Default)]
pub struct Group {
   gid: u16,
   gname: String,
   members: Vec<String>
}

impl Group {
   pub(super) fn new<T: AsRef<str>>(gname: T) -> Result<(), Error> {
      exec_cmd(PKEXEC, vec![GROUP_ADD, gname.as_ref()])?;
      Ok(())
   }

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

   pub fn change_membership(&mut self, ls_members: Vec<&str>) -> Result<(), Error> {
      if ls_members != self.members.iter().map(AsRef::as_ref).collect::<Vec<&str>>() {
         let formatted_members = ls_members.join(",");
         exec_cmd(GPASSWD, vec!["-M", &formatted_members, &self.gname])?;
         self.members = ls_members.iter().map(ToString::to_string).collect();
      }
      Ok(())
   }

   pub fn add_user<T: AsRef<str>>(&mut self, usrname: T) -> Result<(), Error> {
      if !self.members.contains(&usrname.as_ref().to_string()) {
         exec_cmd(GPASSWD, vec!["-a", usrname.as_ref(), &self.gname])?;
         self.members.push(usrname.as_ref().to_string());
      }
      Ok(())
   }

   pub fn remove_user<T: AsRef<str>>(&mut self, usrname: T) -> Result<bool, Error> {
      let mut res = false;
      if self.members.contains(&usrname.as_ref().to_string()) {
         exec_cmd(GPASSWD, vec!["-d", usrname.as_ref(), &self.gname])?;
         let idx = self.members.iter().position(|m| m == &usrname.as_ref().to_string());
         if let Some(idx) = idx {
            self.members.remove(idx);
            res = true;
         }
      }
      Ok(res)
   }

   pub fn append_users(&mut self, ls_members: Vec<&str>) -> Result<(), Error> {
      let mut all_members: Vec<&str> = self.members.iter().map(AsRef::as_ref).chain(ls_members).collect();
      all_members.sort();
      all_members.dedup();
      let formatted_members = all_members.join(",");
      exec_cmd(GPASSWD, vec!["-M", &formatted_members, &self.gname])?;
      self.members = all_members.iter().map(ToString::to_string).collect();
      Ok(())
   }

   pub fn change_name<T: AsRef<str>>(&mut self, new_gname: T) -> Result<(), Error> {
      if new_gname.as_ref() != self.gname.as_str() {
         exec_cmd(GROUP_MOD, vec!["-n", new_gname.as_ref(), self.gname.as_str()])?;
         self.gname = new_gname.as_ref().to_string();
      } 
      Ok(())
   }

   pub fn delete(&mut self) -> Result<(), Error> {
      exec_cmd(GROUP_DEL, vec![&self.gname])?;
      Ok(())
   }

   pub fn gid(&self) -> u16 {
      self.gid
   }

   pub fn name(&self) -> &String {
      &self.gname
   }

   pub fn members(&self) -> &[String] {
      self.members.as_slice()
   }
}