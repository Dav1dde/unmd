use crate::gen::*;
use crate::NetworkManager;
use dbus::blocking::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged;
use dbus::blocking::{Connection, Proxy};
use dbus::Message;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

#[derive(Debug)]
pub enum ActiveConnection {
    Added(String),
    Removed(String),
}

impl ActiveConnection {
    pub fn uuid(self) -> String {
        match self {
            Self::Added(uuid) => uuid,
            Self::Removed(uuid) => uuid,
        }
    }

    pub fn uuid_ref(&self) -> &str {
        match self {
            Self::Added(uuid) => uuid,
            Self::Removed(uuid) => uuid,
        }
    }
}

pub fn monitor(nm: &NetworkManager) -> Receiver<ActiveConnection> {
    let proxy = nm.proxy();
    let (tx, rx) = mpsc::channel();

    ConnectionMonitor::new(tx).connect(&proxy);

    rx
}

struct ConnectionMonitor {
    sender: Sender<ActiveConnection>,
    currently_active: HashMap<String, String>,
}

impl ConnectionMonitor {
    fn new(tx: Sender<ActiveConnection>) -> Self {
        Self {
            sender: tx,
            currently_active: HashMap::new(),
        }
    }

    pub fn connect(mut self, proxy: &Proxy<'_, &Connection>) {
        self.currently_active = Self::currently_active_connections(proxy);

        proxy
            .match_signal(
                move |p: PropertiesPropertiesChanged, c: &Connection, m: &Message| {
                    self.on_properties_changed(p, c, m)
                },
            )
            .unwrap();
    }

    fn currently_active_connections(proxy: &Proxy<'_, &Connection>) -> HashMap<String, String> {
        let mut active_connections = HashMap::new();
        for active_connection in proxy.active_connections().unwrap() {
            let key = active_connection.to_string();
            let proxy = proxy.connection.with_proxy(
                "org.freedesktop.NetworkManager",
                active_connection,
                Duration::from_millis(5000),
            );

            active_connections.insert(key, proxy.uuid().unwrap());
        }
        active_connections
    }

    fn on_properties_changed(
        &mut self,
        p: PropertiesPropertiesChanged,
        connection: &Connection,
        _: &Message,
    ) -> bool {
        let dbus::arg::Variant(active_connections) =
            match p.changed_properties.get("ActiveConnections") {
                Some(active_connections) => active_connections,
                None => return true,
            };

        let active_connections = active_connections
            .as_iter()
            .expect("expected active connections to be iterable")
            .map(|p| p.as_str().expect("expect connection identifier"))
            .collect::<HashSet<_>>();

        self.currently_active
            .iter()
            .filter_map(|(k, v)| (!active_connections.contains(k.as_str())).then(|| v))
            .for_each(|uuid| {
                self.sender
                    .send(ActiveConnection::Removed(uuid.to_string()))
                    .unwrap();
            });
        self.currently_active
            .retain(|k, _| active_connections.contains(k.as_str()));

        for &added_conenction in active_connections.iter() {
            if self.currently_active.contains_key(added_conenction) {
                continue;
            }

            let uuid = connection
                .with_proxy(
                    "org.freedesktop.NetworkManager",
                    added_conenction,
                    Duration::from_millis(5000),
                )
                .uuid()
                .unwrap();

            self.sender
                .send(ActiveConnection::Added(uuid.to_string()))
                .unwrap();
            self.currently_active
                .insert(added_conenction.to_string(), uuid);
        }

        true
    }
}
