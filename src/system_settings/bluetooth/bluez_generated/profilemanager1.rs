// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.ProfileManager1.xml --interfaces=org.bluez.ProfileManager1 --client=nonblock --methodtype=none --prop-newtype`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezProfileManager1 {
    fn register_profile(&self, profile: dbus::Path, uuid: &str, options: arg::PropMap) -> nonblock::MethodReply<()>;
    fn unregister_profile(&self, profile: dbus::Path) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgBluezProfileManager1 for nonblock::Proxy<'a, C> {
    fn register_profile(&self, profile: dbus::Path, uuid: &str, options: arg::PropMap) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.ProfileManager1", "RegisterProfile", (profile, uuid, options))
    }

    fn unregister_profile(&self, profile: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.ProfileManager1", "UnregisterProfile", (profile,))
    }
}

pub const ORG_BLUEZ_PROFILE_MANAGER1_NAME: &str = "org.bluez.ProfileManager1";
