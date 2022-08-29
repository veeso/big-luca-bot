//! # Automatize
//!
//! A module to automatize messages

use crate::big_luca::redis::RedisRepository;

use super::repository::Repository;
use super::rsshub::RssHubClient;
use super::youtube::Youtube;
use super::{AnswerBuilder, Aphorism, Stickers};

use chrono::Utc;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

type AutomatizerResult<T> = Result<T, AutomatizerError>;

/// Automatizer error
#[derive(Debug, Error)]
pub enum AutomatizerError {
    #[error("scheduler error: {0}")]
    Scheduler(JobSchedulerError),
}

impl From<JobSchedulerError> for AutomatizerError {
    fn from(e: JobSchedulerError) -> Self {
        Self::Scheduler(e)
    }
}

/// Automatizer takes care of sending messages to subscribed users
pub struct Automatizer {
    scheduler: JobScheduler,
}

impl Automatizer {
    /// Start automatizer
    pub async fn start() -> AutomatizerResult<Self> {
        debug!("starting automatizer");
        Ok(Self {
            scheduler: Self::setup_cron_scheduler().await?,
        })
    }

    /// Subscribe a chat to the automatizer
    pub async fn subscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        repository.insert_chat(*chat).await?;
        info!("subscribed {} to the automatizer", chat);
        Ok(())
    }

    /// Unsubscribe chat from automatizer. If the chat is not currently subscribed, return error
    pub async fn unsubscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        repository.delete_chat(*chat).await?;
        info!("unsubscribed {} from the automatizer", chat);
        Ok(())
    }

    /// Setup cron scheduler
    async fn setup_cron_scheduler() -> AutomatizerResult<JobScheduler> {
        let sched = JobScheduler::new().await?;
        // daily aphorism job
        let morning_aphorism_job = Job::new_async("0 0 6 * * *", |_, _| {
            Box::pin(async move {
                info!("running morning_aphorism_job");
                if let Err(err) = Self::send_perla().await {
                    error!("evening_aphorism_job failed: {}", err);
                }
            })
        })?;
        sched.add(morning_aphorism_job).await?;
        let afternoon_aphorism_job = Job::new_async("0 30 12 * * *", |_, _| {
            Box::pin(async move {
                info!("running afternoon_aphorism_job");
                if let Err(err) = Self::send_perla().await {
                    error!("afternoon_aphorism_job failed: {}", err);
                }
            })
        })?;
        sched.add(afternoon_aphorism_job).await?;
        // evening aphorism job
        let evening_aphorism_job = Job::new_async("0 30 17 * * *", |_, _| {
            Box::pin(async move {
                info!("running evening_aphorism_job");
                if let Err(err) = Self::send_perla().await {
                    error!("evening_aphorism_job failed: {}", err);
                }
            })
        })?;
        sched.add(evening_aphorism_job).await?;
        // new video check
        let new_video_check_job = Job::new_async("0 30 * * * *", |_, _| {
            Box::pin(async move {
                info!("running new_video_check_job");
                if let Err(err) = Self::fetch_latest_video().await {
                    error!("new_video_check_job failed: {}", err);
                }
            })
        })?;
        sched.add(new_video_check_job).await?;
        // new instagram post job
        let new_instagram_post_job = Job::new_async("0 15 * * * *", |_, _| {
            Box::pin(async move {
                info!("running new_instagram_post_job");
                if let Err(err) = Self::fetch_latest_instagram_post().await {
                    error!("new_instagram_post_job failed: {}", err);
                }
            })
        })?;
        sched.add(new_instagram_post_job).await?;

        sched
            .start()
            .await
            .map(|_| sched)
            .map_err(AutomatizerError::from)
    }

    /// Send perla
    async fn send_perla() -> anyhow::Result<()> {
        let bot = Bot::from_env().auto_send();
        let message = AnswerBuilder::default()
            .text(Aphorism::get_random())
            .sticker(Stickers::random())
            .finalize();
        for chat in Self::subscribed_chats().await?.iter() {
            debug!("sending scheduled aphorism to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to send scheduled aphorism to {}: {}", chat, err);
            }
        }
        Ok(())
    }

    /// Fetch latest video job
    async fn fetch_latest_video() -> anyhow::Result<()> {
        // get last video pub date
        let mut redis_client = RedisRepository::connect()?;
        let last_video_pubdate = redis_client
            .get_last_video_pubdate()
            .await?
            .unwrap_or_else(Utc::now);
        let video = match Youtube::get_oldest_unseen_video(last_video_pubdate).await {
            Ok(Some(v)) => v,
            Ok(None) => {
                debug!("there's no new video to show");
                return Ok(());
            }
            Err(err) => {
                anyhow::bail!("failed to check latest video: {}", err)
            }
        };
        debug!(
            "last time I checked big-luca videos, big-luca video had date {:?}; latest has {:?}",
            last_video_pubdate, video.date
        );
        let date = video.date.unwrap_or_else(Utc::now);
        if last_video_pubdate < date {
            let bot = Bot::from_env().auto_send();
            info!(
                "Big luca published a new video ({:?}): {}",
                video.date,
                video.title.as_deref().unwrap_or_default()
            );
            let message = AnswerBuilder::default()
                .text(format!(
                    "😱😱😱 Il papi ha appena sganciato un nuovo video: {} 💣\n👉 {}",
                    video.title.as_deref().unwrap_or_default(),
                    video.url
                ))
                .sticker(Stickers::luna_e_stelle())
                .finalize();
            for chat in Self::subscribed_chats().await?.iter() {
                debug!("sending new video notify to {}", chat);
                if let Err(err) = message.clone().send(&bot, *chat).await {
                    error!("failed to send scheduled aphorism to {}: {}", chat, err);
                }
            }
            redis_client.set_last_video_pubdate(date).await?;
        }
        Ok(())
    }

    async fn fetch_latest_instagram_post() -> anyhow::Result<()> {
        // get last video pub date
        let mut redis_client = RedisRepository::connect()?;
        let last_post_pubdate = redis_client
            .get_last_instagram_update()
            .await?
            .unwrap_or_else(Utc::now);

        let post = match RssHubClient::get_oldest_unseen_post(last_post_pubdate).await {
            Ok(Some(v)) => v,
            Ok(None) => {
                debug!("There's no new post to see");
                return Ok(());
            }
            Err(err) => {
                anyhow::bail!("failed to check latest ig post: {}", err)
            }
        };
        debug!(
            "last time I checked big-luca ig posts, big-luca ig post had date {:?}; latest has {:?}",
            last_post_pubdate, post.date
        );
        let date = post.date.unwrap_or_else(Utc::now);
        if last_post_pubdate < date {
            let bot = Bot::from_env().auto_send();
            info!(
                "Big luca published a ig post ({:?}): {}",
                post.date,
                post.title.as_deref().unwrap_or_default()
            );
            let message = AnswerBuilder::default()
                .text(format!(
                    "😱😱😱 Il papi ha appena sganciato una nuova perla su instagram: {} 💣\n👉 {}",
                    post.title.as_deref().unwrap_or_default(),
                    post.url
                ))
                .sticker(Stickers::luna_e_stelle())
                .finalize();
            for chat in Self::subscribed_chats().await?.iter() {
                debug!("sending new post notify to {}", chat);
                if let Err(err) = message.clone().send(&bot, *chat).await {
                    error!("failed to send scheduled aphorism to {}: {}", chat, err);
                }
            }
            redis_client.set_last_instagram_update(date).await?;
        }
        Ok(())
    }

    pub async fn subscribed_chats() -> anyhow::Result<Vec<ChatId>> {
        let repository = Repository::connect().await?;
        repository.get_subscribed_chats().await
    }
}

impl Drop for Automatizer {
    fn drop(&mut self) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            info!("Shutting scheduler down");
            if let Err(err) = self.scheduler.shutdown().await {
                error!("failed to stop scheduler: {}", err);
            }
        });
    }
}
