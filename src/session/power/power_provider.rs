use crate::helpers::{dbus_call, dbus_call_init_system, dbus_call_init_system_with_args};
use super::power::Action;
use dbus::{Error, blocking::Connection};
use std::time::Duration;

const UPOWER_SERVICE: &str = "org.freedesktop.UPower";
const UPOWER_PATH: &str = "/org/freedesktop/UPower";
const UPOWER_INTERFACE: &str = UPOWER_SERVICE;

const INIT_SYS_SERVICE: &str = "org.freedesktop.login1";
const INIT_SYS_PATH: &str = "/org/freedesktop/login1";
const INIT_SYS_INTERFACE: &str = "org.freedesktop.login1.Manager";

const LDE_SERVICE: &str = "org.kde.ksmserver";
const LDE_PATH: &str = "/KSMServer";
const LDE_INTERFACE: &str = "org.kde.KSMServerInterface";

pub trait PowerProvider {
    // fn new() -> Self;
    
    fn can_action(&self, action: Action) -> Result<bool, Error>;

    fn do_action(&self, action: Action) -> Result<bool, Error>;
}

pub struct UPowerProvider;

impl PowerProvider for UPowerProvider {
    fn can_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerHibernate => "HibernateAllowed",
            Action::PowerSuspend => "SuspendAllowed",
            _ => return Ok(false),
        };

        let conn = Connection::new_system()?;
        dbus_call_init_system(UPOWER_SERVICE, UPOWER_PATH, UPOWER_INTERFACE, &conn, cmd, None, Duration::from_millis(5000))
    }

    fn do_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerSuspend => "Suspend",
            Action::PowerHibernate => "Hibernate",
            _ => return Ok(false),
        };

        let conn = Connection::new_system()?;
        dbus_call_init_system(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, Some(true), Duration::from_millis(5000))
    }
}

pub struct InitSystemProvider;

impl PowerProvider for InitSystemProvider {
    fn can_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerReboot => "CanReboot",
            Action::PowerShutdown => "CanPowerOff",
            Action::PowerSuspend => "CanSuspend",
            Action::PowerHibernate => "CanHibernate",
            _ => return Ok(false),
        };

        let conn = Connection::new_system()?;
        dbus_call_init_system(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, None, Duration::from_millis(5000))
    }

    fn do_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerReboot => "Reboot",
            Action::PowerShutdown => "PowerOff",
            Action::PowerSuspend => "Suspend",
            Action::PowerHibernate => "Hibernate",
            _ => return Ok(false),
        };

        let conn = Connection::new_system()?;
        dbus_call_init_system(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, Some(true), Duration::from_millis(5000))
    }
}

pub struct Provider;

impl PowerProvider for Provider {
    fn can_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerLogout => "canLogout",
            Action::PowerReboot => "canReboot",
            Action::PowerShutdown => "canPowerOff",
            _ => return Ok(false),
        };
        
        let conn = Connection::new_session()?;
        dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, cmd, Duration::from_millis(5000))
    }

    fn do_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerLogout => "logout",
            Action::PowerReboot => "reboot",
            Action::PowerShutdown => "poweroff",
            _ => return Ok(false),
        };

        let conn = Connection::new_session()?;
        dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, cmd, Duration::from_millis(5000))
    }
}

pub struct SessionProvider;

impl PowerProvider for SessionProvider {
    fn can_action(&self, action: Action) -> Result<bool, Error> { 
        match action {
            Action::PowerLogout => Ok(env!("XDG_SESSION_ID") != "0"),
            _ => Ok(false)
        }
    }

    fn do_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerLogout => "TerminateSession",
            _ => return Ok(false)
        };

        let conn = Connection::new_system()?;
        let pid = env!("XDG_SESSION_ID");
        if pid != "" {
            dbus_call_init_system_with_args(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, Some(pid), Duration::from_millis(5000))
        } else {
            Ok(false)
        }
    }
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// pub enum DbusErrorCheck {
//     CheckDBUS,
//     DontCheckDBUS
// }

// impl Default for DbusErrorCheck {
//     fn default() -> Self {
//         Self::CheckDBUS
//     }
// }