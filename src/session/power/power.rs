use dbus::Error;

use super::power_provider::{
    PowerProvider, InitSystemProvider, Provider, UPowerProvider, SessionProvider,
};

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

pub struct PowerManager {
    providers: Vec<Box<dyn PowerProvider>>
}

impl Default for PowerManager {
    fn default() -> Self {
        Self {providers: Vec::new()}
    }
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            providers: vec![
                // Box::new(Provider),
                Box::new(InitSystemProvider),
                Box::new(UPowerProvider),
                Box::new(SessionProvider)
            ]
        }
    }

    fn can_action(&self, action: Action) -> Result<bool, Error> {
        self.providers.iter().fold(Ok(false), |_, prod| {
            prod.can_action(action)
        })
    }

    fn do_action(&self, action: Action) -> Result<bool, Error> {
        self.providers.iter().fold(Ok(false), |_, prod| {
            match prod.can_action(action) {
                Ok(can_action) => match prod.do_action(action) {
                    Ok(do_action) => Ok(can_action && do_action),
                    Err(err) => Err(err)
                },
                Err(err) => Err(err)
            }
        })
    }

    pub fn can_logout(&self) -> Result<bool, Error> { self.can_action(Action::PowerLogout) }

    pub fn can_hibernate(&self) -> Result<bool, Error> { self.can_action(Action::PowerHibernate) }

    pub fn can_reboot(&self) -> Result<bool, Error> { self.can_action(Action::PowerReboot) }

    pub fn can_shutdown(&self) -> Result<bool, Error> { self.can_action(Action::PowerShutdown) }

    pub fn can_suspend(&self) -> Result<bool, Error> { self.can_action(Action::PowerSuspend) }

    // pub fn can_monitor_off(&self) -> Result<bool, Error> { self.can_action(Action::PowerMonitorOff) }

    pub fn logout(&self) -> Result<bool, Error> { self.do_action(Action::PowerLogout) }

    pub fn hibernate(&self) -> Result<bool, Error> { self.do_action(Action::PowerHibernate) }

    pub fn reboot(&self) -> Result<bool, Error> { self.do_action(Action::PowerReboot) }

    pub fn shutdown(&self) -> Result<bool, Error> { self.do_action(Action::PowerShutdown) }

    pub fn suspend(&self) -> Result<bool, Error> { self.do_action(Action::PowerSuspend) }

    // pub fn monitor_off(&self) -> Result<bool, Error> { self.do_action(Action::PowerMonitorOff) }
}