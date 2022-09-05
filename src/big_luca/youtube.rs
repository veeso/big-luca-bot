//! # Youtube
//!
//! This module exposes the function to fetch the youtube latest videos from big luca

use chrono::{DateTime, Utc};

use crate::feed::{Entry, Feed};
use crate::youtube::YoutubeClient;

// <https://www.youtube.com/feeds/videos.xml?channel_id=UCTpU7OQg9QVsqayEYXTL1LQ>
const CHANNEL_ID: &str = "UCTpU7OQg9QVsqayEYXTL1LQ";

pub struct Youtube;

impl Youtube {
    /// Get latest video from big luca
    pub async fn get_latest_video() -> anyhow::Result<Entry> {
        if let Some(video) = Self::get_latest_videos().await?.entries().next() {
            Ok(video.clone())
        } else {
            anyhow::bail!("Non ho trovato nessun video del Papi. Ma stiamo scherzando!?")
        }
    }

    /// Get oldest unseen post from instagram
    pub async fn get_oldest_unseen_video(
        last_video_pubdate: DateTime<Utc>,
    ) -> anyhow::Result<Option<Entry>> {
        let feed = Self::get_latest_videos().await?;
        // sort by date
        let mut entries: Vec<Entry> = feed.entries().cloned().collect();
        entries.sort_by_key(|x| x.date);
        for entry in entries.into_iter() {
            if entry.date > Some(last_video_pubdate) {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    /// Get latest videos from big luca
    pub async fn get_latest_videos() -> anyhow::Result<Feed> {
        let client = YoutubeClient::new(CHANNEL_ID);
        client
            .fetch()
            .await
            .map_err(|e| anyhow::anyhow!("Impossibile ottenere gli ultimi video del Papi: {}", e))
    }
}
