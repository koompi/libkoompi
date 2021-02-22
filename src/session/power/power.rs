use dbus::Error;

use super::power_provider::{InitSystemProvider, PowerProvider, SessionProvider, UPowerProvider};

/// Power can perform next actions:
#[derive(Debug, Clone, Copy)]
pub enum Action {
   PowerLogout,
   PowerHibernate,
   PowerReboot,
   PowerShutdown,
   PowerSuspend,
   // PowerMonitorOff,
}

/// Structure of Power Manager
pub struct PowerManager {
   providers: Vec<Box<dyn PowerProvider>>,
}

impl Default for PowerManager {
   fn default() -> Self {
      Self { providers: Vec::new() }
   }
}

// Public API
impl PowerManager {
   /// This method is used to initialize the power manager with a bunch of power providers.
   pub fn new() -> Self {
      Self {
         providers: vec![
            // Box::new(Provider),
            Box::new(InitSystemProvider),
            Box::new(UPowerProvider),
            Box::new(SessionProvider),
         ],
      }
   }

   /// This method is used to check if can perform a defined action.
   fn can_action(&self, action: Action) -> Result<bool, Error> {
      self.providers.iter().fold(Ok(false), |_, prod| prod.can_action(action))
   }

   /// This method is used to perform the action after checking success.
   fn do_action(&self, action: Action) -> Result<bool, Error> {
      self.providers.iter().fold(Ok(false), |_, prod| match prod.can_action(action) {
         Ok(can_action) => match prod.do_action(action) {
            Ok(do_action) => Ok(can_action && do_action),
            Err(err) => Err(err),
         },
         Err(err) => Err(err),
      })
   }

   /// This method is used to check if can logout.
   pub fn can_logout(&self) -> Result<bool, Error> {
      self.can_action(Action::PowerLogout)
   }

   /// This method is used to check if can hibernate.
   pub fn can_hibernate(&self) -> Result<bool, Error> {
      self.can_action(Action::PowerHibernate)
   }

   /// This method is used to check if can reboot.
   pub fn can_reboot(&self) -> Result<bool, Error> {
      self.can_action(Action::PowerReboot)
   }

   /// This method is used to check if can shutdown.
   pub fn can_shutdown(&self) -> Result<bool, Error> {
      self.can_action(Action::PowerShutdown)
   }

   /// This method is used to check if can suspend.
   pub fn can_suspend(&self) -> Result<bool, Error> {
      self.can_action(Action::PowerSuspend)
   }

   // pub fn can_monitor_off(&self) -> Result<bool, Error> { self.can_action(Action::PowerMonitorOff) }

   /// This method is used to perform logout with checking.
   pub fn logout(&self) -> Result<bool, Error> {
      self.do_action(Action::PowerLogout)
   }

   /// This method is used to perform hibernate with checking.
   pub fn hibernate(&self) -> Result<bool, Error> {
      self.do_action(Action::PowerHibernate)
   }

   /// This method is used to perform reboot with checking.
   pub fn reboot(&self) -> Result<bool, Error> {
      self.do_action(Action::PowerReboot)
   }

   /// This method is used to perform shutdown with checking
   pub fn shutdown(&self) -> Result<bool, Error> {
      self.do_action(Action::PowerShutdown)
   }

   /// This method is used to perform suspend with checking.
   pub fn suspend(&self) -> Result<bool, Error> {
      self.do_action(Action::PowerSuspend)
   }

   // pub fn monitor_off(&self) -> Result<bool, Error> { self.do_action(Action::PowerMonitorOff) }
}
