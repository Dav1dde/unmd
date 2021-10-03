// This code was autogenerated with `dbus-codegen-rust -s -d org.freedesktop.NetworkManager -p /org/freedesktop/NetworkManager/ActiveConnection/2 -m None -i org.freedesktop.NetworkManager.Connection.Active`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>, )| Ok(r.0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopNetworkManagerConnectionActive {
    fn connection(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn specific_object(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn id(&self) -> Result<String, dbus::Error>;
    fn uuid(&self) -> Result<String, dbus::Error>;
    fn type_(&self) -> Result<String, dbus::Error>;
    fn devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn state_flags(&self) -> Result<u32, dbus::Error>;
    fn default(&self) -> Result<bool, dbus::Error>;
    fn ip4_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn dhcp4_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn default6(&self) -> Result<bool, dbus::Error>;
    fn ip6_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn dhcp6_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn vpn(&self) -> Result<bool, dbus::Error>;
    fn master(&self) -> Result<dbus::Path<'static>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNetworkManagerConnectionActive for blocking::Proxy<'a, C> {

    fn connection(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Connection")
    }

    fn specific_object(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "SpecificObject")
    }

    fn id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Id")
    }

    fn uuid(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Uuid")
    }

    fn type_(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Type")
    }

    fn devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Devices")
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "State")
    }

    fn state_flags(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "StateFlags")
    }

    fn default(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Default")
    }

    fn ip4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Ip4Config")
    }

    fn dhcp4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Dhcp4Config")
    }

    fn default6(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Default6")
    }

    fn ip6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Ip6Config")
    }

    fn dhcp6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Dhcp6Config")
    }

    fn vpn(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Vpn")
    }

    fn master(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Connection.Active", "Master")
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    pub state: u32,
    pub reason: u32,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.state, i);
        arg::RefArg::append(&self.reason, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
            state: i.read()?,
            reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    const NAME: &'static str = "StateChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Connection.Active";
}
