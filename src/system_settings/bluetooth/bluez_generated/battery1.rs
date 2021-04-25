// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.Battery1.xml --interfaces=org.bluez.Battery1 --client=nonblock --methodtype=none --prop-newtype`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezBattery1 {
    fn percentage(&self) -> nonblock::MethodReply<u8>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgBluezBattery1 for nonblock::Proxy<'a, C> {
    fn percentage(&self) -> nonblock::MethodReply<u8> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Battery1", "Percentage")
    }
}

pub const ORG_BLUEZ_BATTERY1_NAME: &str = "org.bluez.Battery1";

#[derive(Copy, Clone, Debug)]
pub struct OrgBluezBattery1Properties<'a>(pub &'a arg::PropMap);

impl<'a> OrgBluezBattery1Properties<'a> {
    pub fn from_interfaces(interfaces: &'a ::std::collections::HashMap<String, arg::PropMap>) -> Option<Self> {
        interfaces.get("org.bluez.Battery1").map(Self)
    }

    pub fn percentage(&self) -> Option<u8> {
        arg::prop_cast(self.0, "Percentage").copied()
    }
}
