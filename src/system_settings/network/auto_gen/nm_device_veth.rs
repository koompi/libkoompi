// This code was autogenerated with `dbus-codegen-rust -s -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceVeth {
    fn peer(&self) -> Result<dbus::Path<'static>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNetworkManagerDeviceVeth for blocking::Proxy<'a, C> {

    fn peer(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Veth", "Peer")
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    pub properties: arg::PropMap,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceVethPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Veth";
}
