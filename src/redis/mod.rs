//! # Redis
//!
//! Redis client module

use redis::{AsyncCommands, Client, FromRedisValue, RedisError, ToRedisArgs};

pub type RedisResult<T> = Result<T, RedisError>;

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    /// Connect to redis
    pub fn connect(url: &str) -> RedisResult<Self> {
        let client = Client::open(url)?;
        debug!("connected to {}", url);
        Ok(Self { client })
    }

    /// Set key
    pub async fn set<V>(&mut self, key: &str, value: V) -> RedisResult<()>
    where
        V: ToRedisArgs + Send + Sync + std::fmt::Debug,
    {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        debug!("SET {} to {:?}", key, value);
        connection.set(key, value).await
    }

    /// Get key
    pub async fn get<V>(&mut self, key: &str) -> RedisResult<Option<V>>
    where
        V: FromRedisValue,
    {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        debug!("GET {}", key);
        connection.get(key).await
    }

    /// Get all keys matching `pattern`
    pub async fn keys(&mut self, pattern: &str) -> RedisResult<Vec<String>> {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        debug!("KEYS {}", pattern);
        connection.keys(pattern).await
    }

    /// Delete key
    pub async fn delete(&mut self, key: &str) -> RedisResult<()> {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        debug!("DEL {}", key);
        connection.del(key).await
    }

    /// Push `value` to list at `key`
    pub async fn list_push<V>(&mut self, key: &str, value: V) -> RedisResult<()>
    where
        V: ToRedisArgs + Send + Sync + std::fmt::Debug,
    {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        debug!("RPUSH {} {:?}", key, value);
        connection.rpush(key, value).await
    }

    /// Get element at `index` from list at `key`
    pub async fn list_get<V>(&mut self, key: &str, index: isize) -> RedisResult<Option<V>>
    where
        V: FromRedisValue,
    {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        debug!("LINDEX {} {}", key, index);
        connection.lindex(key, index).await
    }

    /// Get list length
    pub async fn list_len(&mut self, key: &str) -> RedisResult<usize> {
        let mut connection = self.client.get_multiplexed_async_connection().await?;
        debug!("LLEN {}", key);
        connection.llen(key).await
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn should_set_key() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert!(client.set("test:key1", "1").await.is_ok());
    }

    #[tokio::test]
    async fn should_get_key() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert!(client.set("test:key2", "3").await.is_ok());
        assert_eq!(
            client.get("test:key2").await.unwrap(),
            Some(String::from("3"))
        );
    }

    #[tokio::test]
    async fn should_get_none() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert_eq!(client.get::<String>("test:key3").await.unwrap(), None);
    }

    #[tokio::test]
    async fn should_delete_key() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert!(client.set("test:key2", "3").await.is_ok());
        assert!(client.delete("test:key2").await.is_ok());
        assert!(client.get::<String>("test:key2").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn should_find_keys() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        assert!(client.set("test:search:key1", "3").await.is_ok());
        assert!(client.set("test:search:key2", "3").await.is_ok());
        assert_eq!(client.keys("test:search:*").await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn should_create_list() {
        let mut client = RedisClient::connect("redis://localhost/").unwrap();
        for i in 0..3 {
            assert!(client.list_push("test:list1", i).await.is_ok());
        }
        assert_eq!(
            client
                .list_get::<isize>("test:list1", 0)
                .await
                .unwrap()
                .unwrap(),
            0
        );
        assert_eq!(
            client
                .list_get::<isize>("test:list1", 1)
                .await
                .unwrap()
                .unwrap(),
            1
        );
        assert_eq!(
            client
                .list_get::<isize>("test:list1", 2)
                .await
                .unwrap()
                .unwrap(),
            2
        );
        assert!(client
            .list_get::<isize>("tets::list1", 3)
            .await
            .unwrap()
            .is_none());
    }
}
