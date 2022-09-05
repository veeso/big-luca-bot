//! # Aphorism
//!
//! This module contains the papi's aphorisms

use crate::big_luca::RedisRepository;

use data_encoding::HEXLOWER;
use rand::{seq::SliceRandom, thread_rng};
use ring::digest::{Context, SHA256};
use std::convert::TryFrom;
use teloxide::types::ChatId;

pub struct AphorismJar {
    aphorisms: Vec<String>,
    repository: RedisRepository,
    hash: String,
}

impl AphorismJar {
    /// Get next aphorism for this chat
    pub async fn get_next(&mut self, chat_id: &ChatId) -> anyhow::Result<&'_ str> {
        debug!("getting next aphorism for {}", chat_id);
        let element = self.repository.get_aphorism_jar_element(&chat_id).await?;
        if element.is_none() {
            debug!(
                "could not find any session associated to {}; initializing a new session",
                chat_id
            );
            // create new session for this chat
            self.create_new_session(chat_id).await?;
            info!("created new session for {}", chat_id);
        }
        let element = element.unwrap_or_default();
        let aphorism = match self.aphorisms.get(element) {
            Some(s) => s.as_str(),
            None => anyhow::bail!("impossibile trovare una perla a {}", element),
        };
        self.repository
            .incr_or_reset_aphorism_jar_index(chat_id)
            .await?;
        Ok(aphorism)
    }

    /// Check whether aphorisms have changed for all chats and in case they do, recreate the sequence for that chat
    pub async fn sanitize_aphorisms(aphorisms: &[String]) -> anyhow::Result<()> {
        info!("sanitizing aphorisms");
        let mut jar = Self::try_from(aphorisms)?;
        info!("new aphorisms hash: {}", jar.hash);
        let sessions = jar.repository.get_aphorism_jar_sessions().await?;
        for session in sessions {
            let session_hash = jar
                .repository
                .get_aphorism_jar_session_hash(&session)
                .await?;
            if session_hash.as_deref() != Some(jar.hash.as_str()) {
                debug!(
                    "session {} has a different hash: {:?}; new {}",
                    session, session_hash, jar.hash
                );
                jar.create_new_session(&session).await?;
            }
        }
        Ok(())
    }

    /// Create new session for `chat_id`
    async fn create_new_session(&mut self, chat_id: &ChatId) -> anyhow::Result<()> {
        let mut sequence: Vec<usize> = (0..self.aphorisms.len()).collect();
        sequence.shuffle(&mut thread_rng());
        debug!("created sequence for chat {}: {:?}", chat_id, sequence);
        self.repository
            .create_new_aphorism_jar_session(chat_id, sequence, &self.hash)
            .await
    }

    /// Calculate aphorisms hash
    fn aphorisms_hash(aphorisms: &[String]) -> String {
        let mut digest_ctx = Context::new(&SHA256);
        for aphorism in aphorisms.iter() {
            digest_ctx.update(&aphorism.as_bytes());
        }
        HEXLOWER.encode(digest_ctx.finish().as_ref())
    }
}

impl TryFrom<&[String]> for AphorismJar {
    type Error = anyhow::Error;
    fn try_from(aphorisms: &[String]) -> Result<Self, Self::Error> {
        let repository = RedisRepository::connect()?;
        let aphorisms: Vec<String> = aphorisms.iter().map(|x| x.to_string()).collect();
        let hash = Self::aphorisms_hash(&aphorisms);
        Ok(Self {
            aphorisms,
            repository,
            hash,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::big_luca::Parameters;

    use std::path::Path;

    #[tokio::test]
    async fn should_get_random_aphorism() {
        let parameters = Parameters::try_from_path(Path::new("config/parameters.json")).unwrap();
        let mut aphorism = AphorismJar::try_from(parameters.aphorisms.as_slice()).unwrap();
        assert!(!aphorism.get_next(&ChatId(1)).await.is_ok());
    }
}
