//! # Chat
//!
//! this module contains the chat entity repository

use super::{RepositoryError, RepositoryResult};

use chrono::{DateTime, FixedOffset, Utc};
use sqlx::{Pool, Sqlite};
use teloxide::types::ChatId;

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct Chat {
    id: i64,
    created_at: String,
}

impl Chat {
    pub fn new(chat_id: ChatId) -> Self {
        Self {
            id: chat_id.0,
            created_at: Utc::now().to_rfc3339(),
        }
    }

    /// Return inner `ChatId`
    pub fn id(&self) -> ChatId {
        ChatId(self.id)
    }

    /// Return created_at as a `DateTime`
    pub fn created_at(&self) -> RepositoryResult<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc3339(&self.created_at)
            .map_err(|_| RepositoryError::BadDateTimeSyntax)
    }

    /// Collect all the chat in the database
    pub async fn get_all(db: &Pool<Sqlite>) -> RepositoryResult<Vec<Chat>> {
        sqlx::query_as(
            r#"
            SELECT *
            FROM chat"#,
        )
        .fetch_all(db)
        .await
        .map_err(RepositoryError::from)
    }

    /// Insert `Chat` to database
    pub async fn insert(&self, db: &Pool<Sqlite>) -> RepositoryResult<()> {
        debug!("inserting a new chat {} to repository", self.id);
        let rows = sqlx::query("INSERT INTO chat (id, created_at) VALUES ($1, $2)")
            .bind(self.id)
            .bind(&self.created_at)
            .execute(db)
            .await
            .map_err(RepositoryError::from)?
            .rows_affected();
        if rows != 1 {
            return Err(RepositoryError::TooManyInserts);
        }

        Ok(())
    }

    /// Delete this chat from database
    pub async fn delete(&self, db: &Pool<Sqlite>) -> RepositoryResult<()> {
        debug!("deleting chat {} from repository", self.id);
        sqlx::query("DELETE FROM chat WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await
            .map_err(RepositoryError::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::repository::test::init_database;

    #[tokio::test]
    async fn should_insert_chat() {
        let (db, temp) = init_database().await;
        let chat = Chat::new(ChatId(32));
        assert!(chat.insert(db.pool()).await.is_ok());
        drop(temp)
    }

    #[tokio::test]
    async fn should_delete_chat() {
        let (db, temp) = init_database().await;
        let chat = Chat::new(ChatId(32));
        assert!(chat.insert(db.pool()).await.is_ok());
        assert!(chat.delete(db.pool()).await.is_ok());
        drop(temp)
    }

    #[tokio::test]
    async fn should_retrieve_chat() {
        let (db, temp) = init_database().await;
        let chats = vec![
            Chat::new(ChatId(1)),
            Chat::new(ChatId(2)),
            Chat::new(ChatId(3)),
        ];
        for chat in chats.iter() {
            assert!(chat.insert(db.pool()).await.is_ok());
        }
        // select
        assert_eq!(Chat::get_all(db.pool()).await.unwrap(), chats);
        drop(temp)
    }
}
