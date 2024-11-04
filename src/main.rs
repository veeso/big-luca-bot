const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod big_luca;
mod feed;
mod redis;
mod repository;
mod utils;
mod youtube;

use big_luca::BigLuca;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!(
        "big-luca-bot v{} - developed by {}",
        APP_VERSION, APP_AUTHORS
    );
    let big = BigLuca::init().await?;

    if std::env::var("BIG_LUCA_PIDFILE").is_ok() {
        write_pidfile()?;
    }

    info!("application ready!");
    big.run().await
}

/// Write pidfile
fn write_pidfile() -> anyhow::Result<()> {
    let pid = std::process::id();
    let pidfile = std::env::var("BIG_LUCA_PIDFILE")?;
    std::fs::write(pidfile, pid.to_string())?;
    Ok(())
}
