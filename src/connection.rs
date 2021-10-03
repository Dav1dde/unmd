use crate::gen::{
    OrgFreedesktopNetworkManager, OrgFreedesktopNetworkManagerConnectionActive,
    OrgFreedesktopNetworkManagerConnectionActiveStateChanged, OrgFreedesktopNetworkManagerSettings,
    OrgFreedesktopNetworkManagerSettingsConnection,
};
use crate::NetworkManager;
use dbus::arg::Variant;
use dbus::blocking::Connection as DbusConnection;
use dbus::message::SignalArgs;
use dbus::Message;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};

macro_rules! get_v_str {
    ($connection:ident, $name:expr) => {
        $connection
            .get($name)
            .and_then(|Variant(value)| value.as_str())
            .map(|value| value.to_owned())
    };
}

#[derive(Debug)]
pub struct ConnectionChange {
    pub state: ConnectionState,
    pub connection: Connection,
}

#[derive(Copy, Clone, Debug)]
pub enum ConnectionState {
    Unknown = 0,
    Activating,
    Activated,
    Deactivating,
    Deactivated,
}

pub fn monitor(nm: &NetworkManager) -> Receiver<ConnectionChange> {
    let (tx, rx) = mpsc::channel();

    ConnectionMonitor::new(tx).connect(nm);

    rx
}

pub fn get_connection_by_uuid(nm: &NetworkManager, uuid: impl AsRef<str>) -> Option<Connection> {
    let proxy = nm.with_proxy("/org/freedesktop/NetworkManager/Settings");
    let path = proxy.get_connection_by_uuid(uuid.as_ref()).ok()?;

    let proxy = nm.with_proxy(path);
    let settings = proxy.get_settings().unwrap();
    let connection = settings.get("connection").unwrap();

    let id = get_v_str!(connection, "id").unwrap();
    let uuid = get_v_str!(connection, "uuid").unwrap();
    let r#type = get_v_str!(connection, "type").unwrap();
    let interface_name = get_v_str!(connection, "interface-name");

    Some(Connection {
        id,
        uuid,
        r#type,
        interface_name,
    })
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub id: String,
    pub uuid: String,
    pub r#type: String,
    pub interface_name: Option<String>,
}

struct ConnectionMonitor {
    sender: Sender<ConnectionChange>,
    state: HashMap<String, Connection>,
}

impl ConnectionMonitor {
    fn new(sender: mpsc::Sender<ConnectionChange>) -> Self {
        Self {
            sender,
            state: HashMap::new(),
        }
    }

    fn connect(mut self, nm: &NetworkManager) {
        for active_connection in nm.proxy().active_connections().unwrap() {
            self.update_state(nm.connection(), &active_connection);
        }

        let signal = OrgFreedesktopNetworkManagerConnectionActiveStateChanged::match_rule(
            Some(&"org.freedesktop.NetworkManager".into()),
            None,
        )
        .static_clone();

        nm.connection()
            .add_match(
                signal,
                move |p: OrgFreedesktopNetworkManagerConnectionActiveStateChanged, c, m| {
                    self.on_state_changed(
                        match p.state {
                            0 => ConnectionState::Unknown,
                            1 => ConnectionState::Activating,
                            2 => ConnectionState::Activated,
                            3 => ConnectionState::Deactivating,
                            4 => ConnectionState::Deactivated,
                            _ => panic!("invalid connection state"),
                        },
                        c,
                        m,
                    )
                },
            )
            .unwrap();
    }

    #[tracing::instrument(skip(self, connection))]
    fn update_state(&mut self, connection: &DbusConnection, path: &str) {
        tracing::trace!("updating state");

        let uuid = connection
            .with_proxy(
                "org.freedesktop.NetworkManager",
                path,
                std::time::Duration::from_millis(5000),
            )
            .uuid()
            .unwrap();

        let c = get_connection_by_uuid(&NetworkManager::new(), uuid.to_owned()).unwrap();
        tracing::trace!(uuid = uuid.as_str(), "state update {:?}", c);
        self.state.insert(path.to_owned(), c);
    }

    #[tracing::instrument(skip(self, dbus_connection, message))]
    fn on_state_changed(
        &mut self,
        state: ConnectionState,
        dbus_connection: &DbusConnection,
        message: &Message,
    ) -> bool {
        let path = message.path().unwrap().to_string();
        tracing::trace!(path = path.as_str(), "state change");

        match state {
            ConnectionState::Unknown => return true,
            // When the state changes to deactivated, the active connection
            // does not exist anymore, so we have to fallback to the state.
            ConnectionState::Deactivated => (),
            // Update the state when possible, usually there is a deactivating
            // event before a deactivated signal, which means we have a pretty
            // accurate state here even for the deactivated signal.
            _ => self.update_state(dbus_connection, &path),
        };

        let connection = match self.state.get(&path) {
            Some(connection) => connection,
            None => {
                tracing::warn!(
                    "no information for connection '{}' on state change '{:?}'",
                    path,
                    state
                );
                return true;
            }
        };
        tracing::info!(
            path = path.as_str(),
            "connection update: {} {}",
            connection.id,
            connection.uuid,
        );

        self.sender
            .send(ConnectionChange {
                state,
                connection: connection.clone(),
            })
            .ok()
            .is_some()
    }
}
