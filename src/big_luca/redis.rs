//! # Redis repository client
//!
//! This module exposes the big luca redis repository client

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use teloxide::types::ChatId;

use crate::redis::RedisClient;

use super::Config;

#[cfg(not(test))]
const LAST_VIDEO_PUBDATE: &str = "bigluca-bot:last_video_pubdate";
#[cfg(test)]
const LAST_VIDEO_PUBDATE: &str = "bigluca-bot-test:last_video_pubdate";
#[cfg(not(test))]
const LAST_INSTAGRAM_UPDATE: &str = "bigluca-bot:last_instagram_updatev2";
#[cfg(test)]
const LAST_INSTAGRAM_UPDATE: &str = "bigluca-bot-test:last_instagram_updatev2";
#[cfg(not(test))]
const APHORISM_JAR: &str = "bigluca-bot:aphorism-jar";
#[cfg(test)]
const APHORISM_JAR: &str = "bigluca-bot-test:aphorism-jar";

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
            .get::<String>(LAST_VIDEO_PUBDATE)
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
    pub async fn get_last_instagram_update(&mut self) -> anyhow::Result<Option<SystemTime>> {
        self.redis
            .get::<u64>(LAST_INSTAGRAM_UPDATE)
            .await
            .map_err(|e| anyhow::anyhow!("failed to get last instagram update: {}", e))
            .map(|x| x.map(|x| UNIX_EPOCH.checked_add(Duration::from_secs(x)).unwrap()))
    }

    /// Set last video pubdate
    pub async fn set_last_instagram_update(&mut self, time: SystemTime) -> anyhow::Result<()> {
        self.redis
            .set(
                LAST_INSTAGRAM_UPDATE,
                time.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            )
            .await
            .map_err(|e| anyhow::anyhow!("failed to set last instagram update: {}", e))
    }

    // -- aphorism jar

    /// Push aphorism jar order for chat
    pub async fn push_aphorism_jar_order(
        &mut self,
        chat_id: &ChatId,
        order: &[usize],
    ) -> anyhow::Result<()> {
        // delete key
        let key = Self::aphorism_jar_order_key(chat_id);
        debug!(
            "setting aphorism jar order to {} (len: {})",
            key,
            order.len()
        );
        self.redis.delete(&key).await?;
        for value in order {
            self.redis.list_push(&key, value).await?;
        }
        Ok(())
    }

    /// Get aphorism jar element for this chat
    pub async fn get_aphorism_jar_element(
        &mut self,
        chat_id: &ChatId,
    ) -> anyhow::Result<Option<usize>> {
        let index = self.get_aphorism_jar_index(chat_id).await?;
        let order_key = Self::aphorism_jar_order_key(chat_id);
        debug!("getting aphorism jar element for {}", order_key);
        self.redis
            .list_get(&order_key, index)
            .await
            .map_err(|e| anyhow::anyhow!("failed to get aphorism jar element: {}", e))
    }

    /// Increment or reset aphorism jar index to 0 if exceeds len
    pub async fn incr_or_reset_aphorism_jar_index(
        &mut self,
        chat_id: &ChatId,
    ) -> anyhow::Result<()> {
        let index = self.get_aphorism_jar_index(chat_id).await?;
        let order_key = Self::aphorism_jar_order_key(chat_id);
        let list_len = self.redis.list_len(&order_key).await?;
        // Increment or reset index
        let index = if index as usize >= list_len {
            0
        } else {
            index + 1
        };
        self.set_aphorism_jar_index(chat_id, index).await
    }

    /// Get aphorism jar sessions
    pub async fn get_aphorism_jar_sessions(&mut self) -> anyhow::Result<Vec<ChatId>> {
        let pattern = format!("{}:*:hash", APHORISM_JAR);
        let keys = self.redis.keys(&pattern).await?;
        let mut sessions = Vec::with_capacity(keys.len());
        for key in keys {
            let mut key = key.replace(APHORISM_JAR, "");
            key = key.replace(':', "");
            key = key.replace("hash", "");
            sessions.push(ChatId(key.parse()?));
        }
        Ok(sessions)
    }

    /// Get aphorism jar session hash
    pub async fn get_aphorism_jar_session_hash(
        &mut self,
        chat_id: &ChatId,
    ) -> anyhow::Result<Option<String>> {
        let hash_key = Self::aphorism_jar_hash_key(chat_id);
        self.redis
            .get::<String>(&hash_key)
            .await
            .map_err(|e| anyhow::anyhow!("could not get hash for {}: {}", hash_key, e))
    }

    /// Create new aphorism jar session
    pub async fn create_new_aphorism_jar_session(
        &mut self,
        chat_id: &ChatId,
        sequence: Vec<usize>,
        hash: &str,
    ) -> anyhow::Result<()> {
        info!(
            "creating new aphorism jar session for {} with hash: {}",
            chat_id, hash
        );
        debug!("pushing aphorism jar order: {:?}", sequence);
        self.push_aphorism_jar_order(chat_id, &sequence).await?;
        debug!("setting hash: {}", hash);
        self.set_aphorism_jar_hash(chat_id, hash).await?;
        debug!("setting index to 0");
        self.set_aphorism_jar_index(chat_id, 0).await
    }

    /// Delete aphorism jar associated to chat
    pub async fn delete_aphorism_jar_session(&mut self, chat_id: &ChatId) -> anyhow::Result<()> {
        let pattern = format!("{}:{}:*", APHORISM_JAR, chat_id);
        debug!("deleting keys associated to chat {}", chat_id);
        for key in self.redis.keys(&pattern).await? {
            self.redis.delete(&key).await?;
        }
        info!("deleted aphorism jar session for chat {}", chat_id);
        Ok(())
    }

    /// Set aphorism jar hash
    async fn set_aphorism_jar_hash(&mut self, chat_id: &ChatId, hash: &str) -> anyhow::Result<()> {
        let hash_key = Self::aphorism_jar_hash_key(chat_id);
        self.redis
            .set(&hash_key, hash)
            .await
            .map_err(|e| anyhow::anyhow!("could not set hash for {}: {}", hash_key, e))
    }

    /// Set aphorism jar index
    async fn set_aphorism_jar_index(
        &mut self,
        chat_id: &ChatId,
        index: isize,
    ) -> anyhow::Result<()> {
        let index_key = Self::aphorism_jar_index_key(chat_id);
        self.redis
            .set(&index_key, index)
            .await
            .map_err(|e| anyhow::anyhow!("failed to set index key {}: {}", index_key, e))
    }

    /// Get aphorism jar index
    async fn get_aphorism_jar_index(&mut self, chat_id: &ChatId) -> anyhow::Result<isize> {
        let index_key = Self::aphorism_jar_index_key(chat_id);
        debug!("getting index for aphorism jar for {}", index_key);
        self.redis
            .get::<isize>(&index_key)
            .await
            .map_err(|e| anyhow::anyhow!("failed to get aphorism jar index: {}", e))
            .map(|x| x.unwrap_or_default())
    }

    fn aphorism_jar_order_key(chat_id: &ChatId) -> String {
        format!("{}:{}:order", APHORISM_JAR, chat_id)
    }

    fn aphorism_jar_index_key(chat_id: &ChatId) -> String {
        format!("{}:{}:index", APHORISM_JAR, chat_id)
    }

    fn aphorism_jar_hash_key(chat_id: &ChatId) -> String {
        format!("{}:{}:hash", APHORISM_JAR, chat_id)
    }
}
