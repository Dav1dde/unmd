// This code was autogenerated with `dbus-codegen-rust -s -d org.freedesktop.NetworkManager -p /org/freedesktop/NetworkManager/Settings -f org.freedesktop.NetworkManager.Settings -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerSettings {
    fn list_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn get_connection_by_uuid(&self, uuid: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn add_connection(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error>;
    fn add_connection_unsaved(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error>;
    fn add_connection2(&self, settings: ::std::collections::HashMap<&str, arg::PropMap>, flags: u32, args: arg::PropMap) -> Result<(dbus::Path<'static>, arg::PropMap), dbus::Error>;
    fn load_connections(&self, filenames: Vec<&str>) -> Result<(bool, Vec<String>), dbus::Error>;
    fn reload_connections(&self) -> Result<bool, dbus::Error>;
    fn save_hostname(&self, hostname: &str) -> Result<(), dbus::Error>;
    fn connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn hostname(&self) -> Result<String, dbus::Error>;
    fn can_modify(&self) -> Result<bool, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNetworkManagerSettings for blocking::Proxy<'a, C> {

    fn list_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "ListConnections", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn get_connection_by_uuid(&self, uuid: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "GetConnectionByUuid", (uuid, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn add_connection(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "AddConnection", (connection, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn add_connection_unsaved(&self, connection: ::std::collections::HashMap<&str, arg::PropMap>) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "AddConnectionUnsaved", (connection, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn add_connection2(&self, settings: ::std::collections::HashMap<&str, arg::PropMap>, flags: u32, args: arg::PropMap) -> Result<(dbus::Path<'static>, arg::PropMap), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "AddConnection2", (settings, flags, args, ))
    }

    fn load_connections(&self, filenames: Vec<&str>) -> Result<(bool, Vec<String>), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "LoadConnections", (filenames, ))
    }

    fn reload_connections(&self) -> Result<bool, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "ReloadConnections", ())
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn save_hostname(&self, hostname: &str) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Settings", "SaveHostname", (hostname, ))
    }

    fn connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings", "Connections")
    }

    fn hostname(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings", "Hostname")
    }

    fn can_modify(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Settings", "CanModify")
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerSettingsNewConnection {
    pub connection: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerSettingsNewConnection {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.connection, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerSettingsNewConnection {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerSettingsNewConnection {
            connection: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerSettingsNewConnection {
    const NAME: &'static str = "NewConnection";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    pub connection: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.connection, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
            connection: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerSettingsConnectionRemoved {
    const NAME: &'static str = "ConnectionRemoved";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Settings";
}