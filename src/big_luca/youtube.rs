//! # Youtube
//!
//! This module exposes the function to fetch the youtube latest videos from big luca

use crate::youtube::{Feed, Video, YoutubeClient};

// <https://www.youtube.com/feeds/videos.xml?channel_id=UCTpU7OQg9QVsqayEYXTL1LQ>
const CHANNEL_ID: &str = "UCTpU7OQg9QVsqayEYXTL1LQ";

pub struct Youtube;

impl Youtube {
    /// Get latest video from big luca
    pub async fn get_latest_video() -> anyhow::Result<Video> {
        if let Some(video) = Self::get_latest_videos().await?.videos().next() {
            Ok(video.clone())
        } else {
            anyhow::bail!("Non ho trovato nessun video del Papi. Ma stiamo scherzando!?")
        }
    }

    /// Get latest videos from big luca
    pub async fn get_latest_videos() -> anyhow::Result<Feed> {
        let client = YoutubeClient::new(CHANNEL_ID);
        client
            .fetch()
            .await
            .map_err(|e| anyhow::anyhow!("Impossibile gli ultimi video del Papi: {}", e))
    }
}
