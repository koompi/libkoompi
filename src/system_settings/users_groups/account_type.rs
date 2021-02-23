use std::fmt::{self, Display, Formatter};

/// Variants of User Account Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
   User,
   Admin
}

impl AccountType {
   /// List of available Account Type
   pub const ALL: [AccountType; 2] = [
      AccountType::User,
      AccountType::Admin
   ];
}

impl Default for AccountType {
   fn default() -> AccountType {
      AccountType::User
   }
}

impl Display for AccountType {
   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      write!(
         f,
         "{}",
         match self {
            AccountType::User => "User",
            AccountType::Admin => "Admin",
         }
      )
   }
}