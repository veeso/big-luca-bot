//! # Redis repository client
//!
//! This module exposes the big luca redis repository client

use chrono::{DateTime, Utc};

use crate::redis::RedisClient;

use super::Config;

const LAST_VIDEO_PUBDATE: &str = "bigluca-bot:last_video_pubdate";
const LAST_INSTAGRAM_UPDATE: &str = "bigluca-bot:last_instagram_update";

pub struct RedisRepository {
    redis: RedisClient,
}

impl RedisRepository {
    /// Connect to the database
    pub fn connect() -> anyhow::Result<Self> {
        let config = Config::try_from_env()
            .map_err(|_| anyhow::anyhow!("REDIS_URL is not SET; repository is not available"))?;
        Ok(Self {
            redis: RedisClient::connect(&config.redis_url)
                .map_err(|e| anyhow::anyhow!("failed to connect to redis: {}", e))?,
        })
    }

    /// get last video publication date
    pub async fn get_last_video_pubdate(&mut self) -> anyhow::Result<Option<DateTime<Utc>>> {
        self.redis
            .get(LAST_VIDEO_PUBDATE)
            .await
            .map_err(|e| anyhow::anyhow!("failed to get last video pubdate: {}", e))
            .map(|x| {
                x.and_then(|x| {
                    DateTime::parse_from_rfc3339(&x)
                        .ok()
                        .map(|x| DateTime::from_utc(x.naive_utc(), Utc))
                })
            })
    }

    /// Set last video pubdate
    pub async fn set_last_video_pubdate(&mut self, date: DateTime<Utc>) -> anyhow::Result<()> {
        self.redis
            .set(LAST_VIDEO_PUBDATE, date.to_rfc3339().as_str())
            .await
            .map_err(|e| anyhow::anyhow!("failed to set last video pubdate: {}", e))
    }

    /// get last video publication date
    pub async fn get_last_instagram_update(&mut self) -> anyhow::Result<Option<DateTime<Utc>>> {
        self.redis
            .get(LAST_INSTAGRAM_UPDATE)
            .await
            .map_err(|e| anyhow::anyhow!("failed to get last instagram update: {}", e))
            .map(|x| {
                x.and_then(|x| {
                    DateTime::parse_from_rfc3339(&x)
                        .ok()
                        .map(|x| DateTime::from_utc(x.naive_utc(), Utc))
                })
            })
    }

    /// Set last video pubdate
    pub async fn set_last_instagram_update(&mut self, date: DateTime<Utc>) -> anyhow::Result<()> {
        self.redis
            .set(LAST_INSTAGRAM_UPDATE, date.to_rfc3339().as_str())
            .await
            .map_err(|e| anyhow::anyhow!("failed to set last instagram update: {}", e))
    }
}
