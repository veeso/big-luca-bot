//! # Rsshub
//!
//! Big luca bot rsshub helper

use chrono::{DateTime, Utc};

use crate::{
    feed::{Entry, Feed},
    rsshub::RssHubClient as Client,
};

use super::Config;

pub struct RssHubClient;

const BIGLUCA_ACCOUNT_ID: &str = "bigluca.marketing";

impl RssHubClient {
    /// Get newest (latest) post from instagram
    pub async fn get_latest_post() -> anyhow::Result<Entry> {
        let feed = Self::get_posts().await?;
        match feed.entries().next() {
            Some(e) => Ok(e.clone()),
            None => anyhow::bail!("Non ho trovato nessun post su Instagram da parte del papi."),
        }
    }

    /// Get oldest unseen post from instagram
    pub async fn get_oldest_unseen_post(
        last_post_pubdate: DateTime<Utc>,
    ) -> anyhow::Result<Option<Entry>> {
        let feed = Self::get_posts().await?;
        // sort by date
        let mut entries: Vec<Entry> = feed.entries().cloned().collect();
        entries.sort_by_key(|x| x.date);
        for entry in entries.into_iter() {
            if entry.date > Some(last_post_pubdate) {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    /// Get posts
    async fn get_posts() -> anyhow::Result<Feed> {
        let config = Config::try_from_env()
            .map_err(|_| anyhow::anyhow!("RSSHUB_URL is not SET; repository is not available"))?;
        Client::new(config.rsshub_url)
            .fetch_picuki(BIGLUCA_ACCOUNT_ID)
            .await
            .map_err(|e| anyhow::anyhow!("failed to fetch big luca instagram account: {}", e))
    }
}
