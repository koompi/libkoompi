// This code was autogenerated with `dbus-codegen-rust --file=org.bluez.Device1.xml --interfaces=org.bluez.Device1 --methodtype=none --prop-newtype -o device1.rs`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgBluezDevice1 {
    fn disconnect(&self) -> Result<(), dbus::Error>;
    fn connect(&self) -> Result<(), dbus::Error>;
    fn connect_profile(&self, uuid: &str) -> Result<(), dbus::Error>;
    fn disconnect_profile(&self, uuid: &str) -> Result<(), dbus::Error>;
    fn pair(&self) -> Result<(), dbus::Error>;
    fn cancel_pairing(&self) -> Result<(), dbus::Error>;
    fn address(&self) -> Result<String, dbus::Error>;
    fn address_type(&self) -> Result<String, dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn alias(&self) -> Result<String, dbus::Error>;
    fn set_alias(&self, value: String) -> Result<(), dbus::Error>;
    fn class(&self) -> Result<u32, dbus::Error>;
    fn appearance(&self) -> Result<u16, dbus::Error>;
    fn icon(&self) -> Result<String, dbus::Error>;
    fn paired(&self) -> Result<bool, dbus::Error>;
    fn trusted(&self) -> Result<bool, dbus::Error>;
    fn set_trusted(&self, value: bool) -> Result<(), dbus::Error>;
    fn blocked(&self) -> Result<bool, dbus::Error>;
    fn set_blocked(&self, value: bool) -> Result<(), dbus::Error>;
    fn legacy_pairing(&self) -> Result<bool, dbus::Error>;
    fn rssi(&self) -> Result<i16, dbus::Error>;
    fn connected(&self) -> Result<bool, dbus::Error>;
    fn uuids(&self) -> Result<Vec<String>, dbus::Error>;
    fn modalias(&self) -> Result<String, dbus::Error>;
    fn adapter(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn manufacturer_data(&self) -> Result<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn service_data(&self) -> Result<arg::PropMap, dbus::Error>;
    fn tx_power(&self) -> Result<i16, dbus::Error>;
    fn services_resolved(&self) -> Result<bool, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgBluezDevice1 for blocking::Proxy<'a, C> {

    fn disconnect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Disconnect", ())
    }

    fn connect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Connect", ())
    }

    fn connect_profile(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "ConnectProfile", (uuid, ))
    }

    fn disconnect_profile(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "DisconnectProfile", (uuid, ))
    }

    fn pair(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Pair", ())
    }

    fn cancel_pairing(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "CancelPairing", ())
    }

    fn address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Address")
    }

    fn address_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "AddressType")
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Name")
    }

    fn alias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Alias")
    }

    fn class(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Class")
    }

    fn appearance(&self) -> Result<u16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Appearance")
    }

    fn icon(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Icon")
    }

    fn paired(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Paired")
    }

    fn trusted(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Trusted")
    }

    fn blocked(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Blocked")
    }

    fn legacy_pairing(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "LegacyPairing")
    }

    fn rssi(&self) -> Result<i16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "RSSI")
    }

    fn connected(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Connected")
    }

    fn uuids(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "UUIDs")
    }

    fn modalias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Modalias")
    }

    fn adapter(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Adapter")
    }

    fn manufacturer_data(&self) -> Result<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ManufacturerData")
    }

    fn service_data(&self) -> Result<arg::PropMap, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ServiceData")
    }

    fn tx_power(&self) -> Result<i16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "TxPower")
    }

    fn services_resolved(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ServicesResolved")
    }

    fn set_alias(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Alias", value)
    }

    fn set_trusted(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Trusted", value)
    }

    fn set_blocked(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Blocked", value)
    }
}

pub const ORG_BLUEZ_DEVICE1_NAME: &str = "org.bluez.Device1";

#[derive(Copy, Clone, Debug)]
pub struct OrgBluezDevice1Properties<'a>(pub &'a arg::PropMap);

impl<'a> OrgBluezDevice1Properties<'a> {
    pub fn from_interfaces(
        interfaces: &'a ::std::collections::HashMap<String, arg::PropMap>,
    ) -> Option<Self> {
        interfaces.get("org.bluez.Device1").map(Self)
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

    pub fn appearance(&self) -> Option<u16> {
        arg::prop_cast(self.0, "Appearance").copied()
    }

    pub fn icon(&self) -> Option<&String> {
        arg::prop_cast(self.0, "Icon")
    }

    pub fn paired(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Paired").copied()
    }

    pub fn trusted(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Trusted").copied()
    }

    pub fn blocked(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Blocked").copied()
    }

    pub fn legacy_pairing(&self) -> Option<bool> {
        arg::prop_cast(self.0, "LegacyPairing").copied()
    }

    pub fn rssi(&self) -> Option<i16> {
        arg::prop_cast(self.0, "RSSI").copied()
    }

    pub fn connected(&self) -> Option<bool> {
        arg::prop_cast(self.0, "Connected").copied()
    }

    pub fn uuids(&self) -> Option<&Vec<String>> {
        arg::prop_cast(self.0, "UUIDs")
    }

    pub fn modalias(&self) -> Option<&String> {
        arg::prop_cast(self.0, "Modalias")
    }

    pub fn adapter(&self) -> Option<&dbus::Path<'static>> {
        arg::prop_cast(self.0, "Adapter")
    }

    pub fn manufacturer_data(&self) -> Option<&::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>> {
        arg::prop_cast(self.0, "ManufacturerData")
    }

    pub fn service_data(&self) -> Option<&arg::PropMap> {
        arg::prop_cast(self.0, "ServiceData")
    }

    pub fn tx_power(&self) -> Option<i16> {
        arg::prop_cast(self.0, "TxPower").copied()
    }

    pub fn services_resolved(&self) -> Option<bool> {
        arg::prop_cast(self.0, "ServicesResolved").copied()
    }
}
