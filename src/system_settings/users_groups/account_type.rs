use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
   Normal,
   Admin
}

impl AccountType {
   const ALL: [AccountType; 2] = [
      AccountType::Normal,
      AccountType::Admin
   ];
}

impl Default for AccountType {
   fn default() -> AccountType {
      AccountType::Normal
   }
}

impl Display for AccountType {
   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      write!(
         f,
         "{}",
         match self {
            AccountType::Normal => "Normal User",
            AccountType::Admin => "Administrator",
         }
      )
   }
}