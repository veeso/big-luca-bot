//! # Repository
//!
//! This module contains the trait and the model to implement to interact with the repository

pub mod chat;
use sqlx::sqlite::SqlitePool;
use thiserror::Error;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("unexpected inserts count")]
    TooManyInserts,
    #[error("datetime has an invalid syntax")]
    BadDateTimeSyntax,
    #[error("database error: {0}")]
    Db(sqlx::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(e: sqlx::Error) -> Self {
        Self::Db(e)
    }
}

pub struct SqliteDb {
    pool: SqlitePool,
}

impl SqliteDb {
    /// Connect to sqlite db
    pub async fn connect(database_url: &str) -> RepositoryResult<Self> {
        info!("opening database at {}", database_url);
        let db = SqlitePool::connect(database_url)
            .await
            .map_err(RepositoryError::from)
            .map(|pool| Self { pool })?;
        db.init_tables().await?;
        Ok(db)
    }

    /// Access db pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Init db tables
    async fn init_tables(&self) -> RepositoryResult<()> {
        debug!("initializing tables");
        // chat table
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS chat (
            id INTEGER PRIMARY KEY,
            created_at TEXT
          );"#,
        )
        .execute(self.pool())
        .await
        .map_err(RepositoryError::from)
        .map(|_| ())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn should_init_sqlite3_database() {
        let _ = init_database().await;
    }

    pub async fn init_database() -> (SqliteDb, NamedTempFile) {
        let temp = NamedTempFile::new().expect("failed to create tempfile");
        let pool = SqliteDb::connect(&temp.path().to_string_lossy())
            .await
            .expect("failed to connect to database");
        (pool, temp)
    }
}
