// This code was autogenerated with `dbus-codegen-rust -s -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceWifiP2P {
    fn start_find(&self, options: arg::PropMap) -> Result<(), dbus::Error>;
    fn stop_find(&self) -> Result<(), dbus::Error>;
    fn hw_address(&self) -> Result<String, dbus::Error>;
    fn peers(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNetworkManagerDeviceWifiP2P for blocking::Proxy<'a, C> {

    fn start_find(&self, options: arg::PropMap) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device.WifiP2P", "StartFind", (options, ))
    }

    fn stop_find(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device.WifiP2P", "StopFind", ())
    }

    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.WifiP2P", "HwAddress")
    }

    fn peers(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.WifiP2P", "Peers")
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWifiP2PPeerAdded {
    pub peer: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWifiP2PPeerAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.peer, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWifiP2PPeerAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWifiP2PPeerAdded {
            peer: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWifiP2PPeerAdded {
    const NAME: &'static str = "PeerAdded";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.WifiP2P";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWifiP2PPeerRemoved {
    pub peer: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWifiP2PPeerRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.peer, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWifiP2PPeerRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWifiP2PPeerRemoved {
            peer: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWifiP2PPeerRemoved {
    const NAME: &'static str = "PeerRemoved";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.WifiP2P";
}
