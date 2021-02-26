// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.Media1.xml --interfaces=org.bluez.Media1 --client=nonblock --methodtype=none --prop-newtype`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezMedia1 {
    fn register_endpoint(&self, endpoint: dbus::Path, properties: arg::PropMap) -> nonblock::MethodReply<()>;
    fn unregister_endpoint(&self, endpoint: dbus::Path) -> nonblock::MethodReply<()>;
    fn register_player(&self, player: dbus::Path, properties: arg::PropMap) -> nonblock::MethodReply<()>;
    fn unregister_player(&self, player: dbus::Path) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgBluezMedia1 for nonblock::Proxy<'a, C> {
    fn register_endpoint(&self, endpoint: dbus::Path, properties: arg::PropMap) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Media1", "RegisterEndpoint", (endpoint, properties))
    }

    fn unregister_endpoint(&self, endpoint: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Media1", "UnregisterEndpoint", (endpoint,))
    }

    fn register_player(&self, player: dbus::Path, properties: arg::PropMap) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Media1", "RegisterPlayer", (player, properties))
    }

    fn unregister_player(&self, player: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.Media1", "UnregisterPlayer", (player,))
    }
}

pub const ORG_BLUEZ_MEDIA1_NAME: &str = "org.bluez.Media1";
