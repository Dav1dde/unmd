use std::env;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let nm = unmd::NetworkManager::new();

    let directory = Path::new(&env::var("HOME")?).join(".config").join("unmd");
    tracing::info!("using scripts directory: {:?}", directory);
    let sr = unmd::script::ScriptRunner::new(directory);

    let rx = unmd::connection::monitor(&nm);

    let _guard = nm.start();

    loop {
        let connection_change = rx.recv()?;
        sr.run(&connection_change);
    }
}
