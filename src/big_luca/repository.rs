//! # Big luca bot repository
//!
//! This module contains the interface to the bot repository

use crate::repository::{chat::Chat, SqliteDb};

use teloxide::types::ChatId;

pub struct Repository {
    db: SqliteDb,
}

impl Repository {
    /// Connect to the database
    pub async fn connect() -> anyhow::Result<Self> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL; repository is not available"))?;
        Ok(Self {
            db: SqliteDb::connect(&url)
                .await
                .map_err(|e| anyhow::anyhow!("failed to connect to the database: {}", e))?,
        })
    }

    /// Insert a chat to database
    pub async fn insert_chat(&self, chat: ChatId) -> anyhow::Result<()> {
        Chat::new(chat)
            .insert(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to insert chat into the database: {}", e))
    }

    /// Delete chat from database
    pub async fn delete_chat(&self, chat: ChatId) -> anyhow::Result<()> {
        Chat::new(chat)
            .delete(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to delete chat from the database: {}", e))
    }

    /// Get subscribed chats
    pub async fn get_subscribed_chats(&self) -> anyhow::Result<Vec<ChatId>> {
        Chat::get_all(self.db.pool())
            .await
            .map_err(|e| anyhow::anyhow!("failed to collect subscribed chats: {}", e))
            .map(|x| {
                x.into_iter()
                    .map(|x| {
                        debug!(
                            "found subscribed chat {} ({})",
                            x.id(),
                            x.created_at()
                                .map(|x| x.to_rfc3339())
                                .unwrap_or(String::from("date error"))
                        );
                        x.id()
                    })
                    .collect()
            })
    }
}
