// This code was autogenerated with `dbus-codegen-rust -s -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerSettingsConnection {
    fn update(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error>;
    fn update_unsaved(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error>;
    fn delete(&self) -> Result<(), dbus::Error>;
    fn get_settings(&self) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error>;
    fn get_secrets(&self, setting_name: &str) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error>;
    fn clear_secrets(&self) -> Result<(), dbus::Error>;
    fn save(&self) -> Result<(), dbus::Error>;
    fn update2(&self, settings: ::std::collections::HashMap<&str, arg::PropMap>, flags: u32, args: arg::PropMap) -> Result<arg::PropMap, dbus::Error>;
    fn unsaved(&self) -> Result<bool, dbus::Error>;
    fn flags(&self) -> Result<u32, dbus::Error>;
    fn filename(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNetworkManagerSettingsConnection for blocking::Proxy<'a, C> {

    fn update(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Update", (properties, ))
    }

    fn update_unsaved(&self, properties: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "UpdateUnsaved", (properties, ))
    }

    fn delete(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Delete", ())
    }

    fn get_settings(&self) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "GetSettings", ())
            .and_then(|r: (::std::collections::HashMap<String, arg::PropMap>, )| Ok(r.0, ))
    }

    fn get_secrets(&self, setting_name: &str) -> Result<::std::collections::HashMap<String, arg::PropMap>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "GetSecrets", (setting_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::PropMap>, )| Ok(r.0, ))
    }

    fn clear_secrets(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "ClearSecrets", ())
    }

    fn save(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Save", ())
    }

    fn update2(&self, settings: ::std::collections::HashMap<&str, arg::PropMap>, flags: u32, args: arg::PropMap) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings.Connection", "Update2", (settings, flags, args, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }

    fn unsaved(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings.Connection", "Unsaved")
    }

    fn flags(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings.Connection", "Flags")
    }

    fn filename(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings.Connection", "Filename")
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerSettingsConnectionUpdated {
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerSettingsConnectionUpdated {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerSettingsConnectionUpdated {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerSettingsConnectionUpdated {
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerSettingsConnectionUpdated {
    const NAME: &'static str = "Updated";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    const NAME: &'static str = "Removed";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerSettingsConnectionPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerSettingsConnectionPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerSettingsConnectionPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerSettingsConnectionPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerSettingsConnectionPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings.Connection";
}
