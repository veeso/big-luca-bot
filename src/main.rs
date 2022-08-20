const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tracing;

mod big_luca;
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
    info!("application ready!");
    big.run().await
}
