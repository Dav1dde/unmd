#[allow(clippy::all, warnings)]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let nm = unmd::NetworkManager::new();

    let rx = unmd::connection::monitor(&nm);

    let _guard = nm.start();

    let nm = unmd::NetworkManager::new();

    loop {
        match rx.recv() {
            Ok(ok) => {
                dbg!(&ok);
                // dbg!(unmd::connection::get_connection_by_uuid(&nm, ok.uuid()));
            }
            Err(err) => {
                dbg!(err);
                break;
            }
        };
    }

    Ok(())
}
