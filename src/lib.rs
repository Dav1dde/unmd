pub mod active_connection;
pub mod connection;
pub mod gen;
pub mod script;

use dbus::blocking::{Connection, Proxy};
use dbus::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct NetworkManager {
    connection: Connection,
}

impl NetworkManager {
    pub fn new() -> Self {
        let connection = Connection::new_system().unwrap();

        Self { connection }
    }

    pub fn start(self) -> NetworkManagerHandle {
        let (guard, stop) = mpsc::channel();

        let jh = thread::spawn(move || loop {
            self.connection
                .process(Duration::from_millis(500))
                .expect("process dbus connection");
            if let Err(mpsc::TryRecvError::Disconnected) = stop.try_recv() {
                break;
            }
        });

        NetworkManagerHandle(guard, jh)
    }

    pub fn proxy(&self) -> Proxy<'_, &Connection> {
        self.with_proxy("/org/freedesktop/NetworkManager")
    }

    pub fn with_proxy<'s, 'a>(&'s self, path: impl Into<Path<'a>>) -> Proxy<'a, &'s Connection> {
        self.connection.with_proxy(
            "org.freedesktop.NetworkManager",
            path,
            Duration::from_millis(5000),
        )
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NetworkManagerHandle(mpsc::Sender<()>, thread::JoinHandle<()>);

impl NetworkManagerHandle {
    pub fn join(self) -> thread::Result<()> {
        self.1.join()
    }
}
