// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.Adapter1.xml --interfaces=org.bluez.Adapter1 --client=nonblock --methodtype=none --prop-newtype`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;
pub trait OrgBluezAdapter1 {
    fn start_discovery(&self) -> nonblock::MethodReply<()>;
    fn set_discovery_filter(&self, properties: arg::PropMap) -> nonblock::MethodReply<()>;
    fn stop_discovery(&self) -> nonblock::MethodReply<()>;
    fn remove_device(&self, device: dbus::Path) -> nonblock::MethodReply<()>;
    fn get_discovery_filters(&self) -> nonblock::MethodReply<Vec<String>>;
    fn address(&self) -> nonblock::MethodReply<String>;
    fn address_type(&self) -> nonblock::MethodReply<String>;
    fn name(&self) -> nonblock::MethodReply<String>;
    fn alias(&self) -> nonblock::MethodReply<String>;
    fn set_alias(&self, value: String) -> nonblock::MethodReply<()>;
    fn class(&self) -> nonblock::MethodReply<u32>;
    fn powered(&self) -> nonblock::MethodReply<bool>;
    fn set_powered(&self, value: bool) -> nonblock::MethodReply<()>;
    fn discoverable(&self) -> nonblock::MethodReply<bool>;
    fn set_discoverable(&self, value: bool) -> nonblock::MethodReply<()>;
    fn discoverable_timeout(&self) -> nonblock::MethodReply<u32>;
    fn set_discoverable_timeout(&self, value: u32) -> nonblock::MethodReply<()>;
    fn pairable(&self) -> nonblock::MethodReply<bool>;
    fn set_pairable(&self, value: bool) -> nonblock::MethodReply<()>;
    fn pairable_timeout(&self) -> nonblock::MethodReply<u32>;
    fn set_pairable_timeout(&self, value: u32) -> nonblock::MethodReply<()>;
    fn discovering(&self) -> nonblock::MethodReply<bool>;
    fn uuids(&self) -> nonblock::MethodReply<Vec<String>>;
    fn modalias(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgBluezAdapter1 for nonblock::Proxy<'a, C> {
    fn start_discovery(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Adapter1", "StartDiscovery", ())
    }

    fn set_discovery_filter(&self, properties: arg::PropMap) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Adapter1", "SetDiscoveryFilter", (properties,))
    }

    fn stop_discovery(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Adapter1", "StopDiscovery", ())
    }

    fn remove_device(&self, device: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Adapter1", "RemoveDevice", (device,))
    }

    fn get_discovery_filters(&self) -> nonblock::MethodReply<Vec<String>> {
        self.method_call("org.bluez.Adapter1", "GetDiscoveryFilters", ()).and_then(|r: (Vec<String>,)| Ok(r.0))
    }

    fn address(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Address")
    }

    fn address_type(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "AddressType")
    }

    fn name(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Name")
    }

    fn alias(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Alias")
    }

    fn class(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Class")
    }

    fn powered(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Powered")
    }

    fn discoverable(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discoverable")
    }

    fn discoverable_timeout(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "DiscoverableTimeout")
    }

    fn pairable(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Pairable")
    }

    fn pairable_timeout(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "PairableTimeout")
    }

    fn discovering(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discovering")
    }

    fn uuids(&self) -> nonblock::MethodReply<Vec<String>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "UUIDs")
    }

    fn modalias(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Modalias")
    }

    fn set_alias(&self, value: String) -> nonblock::MethodReply<()> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Alias", value)
    }

    fn set_powered(&self, value: bool) -> nonblock::MethodReply<()> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Powered", value)
    }

    fn set_discoverable(&self, value: bool) -> nonblock::MethodReply<()> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Discoverable", value)
    }

    fn set_discoverable_timeout(&self, value: u32) -> nonblock::MethodReply<()> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "DiscoverableTimeout", value)
    }

    fn set_pairable(&self, value: bool) -> nonblock::MethodReply<()> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Pairable", value)
    }

    fn set_pairable_timeout(&self, value: u32) -> nonblock::MethodReply<()> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "PairableTimeout", value)
    }
}

pub const ORG_BLUEZ_ADAPTER1_NAME: &str = "org.bluez.Adapter1";

#[derive(Copy, Clone, Debug)]
pub struct OrgBluezAdapter1Properties<'a>(pub &'a arg::PropMap);

impl<'a> OrgBluezAdapter1Properties<'a> {
    pub fn from_interfaces(interfaces: &'a ::std::collections::HashMap<String, arg::PropMap>) -> Option<Self> {
        interfaces.get("org.bluez.Adapter1").map(Self)
    }

    pub fn address(&self) -> Option<&String> {
        arg::prop_cast(self.0, "Address")
    }

    pub fn address_type(&self) -> Option<&String> {
        arg::prop_cast(self.0, "AddressType")
    }

    pub fn name(&self) -> Option<&String> {
        arg::prop_cast(self.0, "Name")
    }

    pub fn alias(&self) -> Option<&String> {
        arg::prop_cast(self.0, "Alias")
    }

    pub fn class(&self) -> Option<u32> {
        arg::prop_cast(self.0, "Class").copied()
    }

    pub fn powered(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Powered").copied()
    }

    pub fn discoverable(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Discoverable").copied()
    }

    pub fn discoverable_timeout(&self) -> Option<u32> {
        arg::prop_cast(self.0, "DiscoverableTimeout").copied()
    }

    pub fn pairable(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Pairable").copied()
    }

    pub fn pairable_timeout(&self) -> Option<u32> {
        arg::prop_cast(self.0, "PairableTimeout").copied()
    }

    pub fn discovering(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Discovering").copied()
    }

    pub fn uuids(&self) -> Option<&Vec<String>> {
        arg::prop_cast(self.0, "UUIDs")
    }

    pub fn modalias(&self) -> Option<&String> {
        arg::prop_cast(self.0, "Modalias")
    }
}