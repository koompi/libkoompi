#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConnectionState {
    Unknown = 0,
    Activating = 1,
    Activated = 2,
    Deactivating = 3,
    Deactivated = 4,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeviceType {
    Unknown,
    Ethernet,
    WiFi,
    Unused1,
    Unused2,
    Bt,
    OlpcMesh,
    Wimax,
    Modem,
    Infiniband,
    Bond,
    Vlan,
    Adsl,
    Bridge,
    Generic,
    Team,
    Tun,
    IpTunnel,
    Macvlan,
    Vxlan,
    Veth,
    Macsec,
    Dummy,
}
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceState {
    Unknown,
    Unmanaged,
    Unavailable,
    Disconnected,
    Prepare,
    Config,
    NeedAuth,
    IpConfig,
    IpCheck,
    Secondaries,
    Activated,
    Deactivating,
    Failed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NetworkManagerState {
    Unknown,
    Asleep,
    Disconnected,
    Disconnecting,
    Connecting,
    ConnectedLocal,
    ConnectedSite,
    ConnectedGlobal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Connectivity {
    Unknown,
    None,
    Portal,
    Limited,
    Full,
}
#[derive(Debug, Eq, PartialEq)]
pub enum ServiceState {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
}

#[derive(Debug)]
pub enum ActiveConnectionState {
    Unknown = 0,
    Activating = 1,
    Activated = 2,
    Deactivating = 3,
    Deactivated = 4,
}
