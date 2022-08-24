//! # Rsshub
//!
//! Big luca bot rsshub helper

use crate::{feed::Entry, rsshub::RssHubClient as Client};

use super::Config;

pub struct RssHubClient;

const BIGLUCA_ACCOUNT_ID: &str = "bigluca.marketing";

impl RssHubClient {
    pub async fn get_latest_post() -> anyhow::Result<Entry> {
        let config = Config::try_from_env()
            .map_err(|_| anyhow::anyhow!("RSSHUB_URL is not SET; repository is not available"))?;
        let feed = Client::new(config.rsshub_url)
            .fetch_picuki(BIGLUCA_ACCOUNT_ID)
            .await
            .map_err(|e| anyhow::anyhow!("failed to fetch big luca instagram account: {}", e))?;
        match feed.entries().next() {
            Some(e) => Ok(e.clone()),
            None => anyhow::bail!("Non ho trovato nessun post su Instagram da parte del papi."),
        }
    }
}
