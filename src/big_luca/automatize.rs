//! # Automatize
//!
//! A module to automatize messages

use super::repository::Repository;
use super::youtube::Youtube;
use super::{AnswerBuilder, Aphorism, Stickers};

use chrono::{DateTime, Local};
use futures::lock::Mutex;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

type AutomatizerResult<T> = Result<T, AutomatizerError>;

lazy_static! {
    static ref LAST_VIDEO_PUBLISHED_DATE: Mutex<DateTime<Local>> = Mutex::new(DateTime::default());
}

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
        if let Err(err) = Self::notify_started().await {
            error!("failed to send start notify: {}", err);
        }
        Ok(Self {
            scheduler: Self::setup_cron_scheduler().await?,
        })
    }

    /// Subscribe a chat to the automatizer
    pub async fn subscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        info!("subscribed {} to the automatizer", chat);
        let repository = Repository::connect().await?;
        repository.insert_chat(*chat).await
    }

    /// Unsubscribe chat from automatizer. If the chat is not currently subscribed, return error
    pub async fn unsubscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        repository.delete_chat(*chat).await
    }

    /// Setup cron scheduler
    async fn setup_cron_scheduler() -> AutomatizerResult<JobScheduler> {
        let sched = JobScheduler::new().await?;
        // daily aphorism job
        let morning_aphorism_job = Job::new_async("0 0 7 * * *", |_, _| {
            Box::pin(async move {
                info!("running morning_aphorism_job");
                if let Err(err) = Self::send_perla().await {
                    error!("evening_aphorism_job failed: {}", err);
                }
            })
        })?;
        sched.add(morning_aphorism_job).await?;
        // evening aphorism job
        let evening_aphorism_job = Job::new_async("0 0 18 * * *", |_, _| {
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
        let video = match Youtube::get_latest_video().await {
            Ok(v) => v,
            Err(err) => {
                anyhow::bail!("failed to check latest video: {}", err)
            }
        };
        if let Some(date) = video.date {
            debug!(
                "last time I checked big-luca videos, big-luca video had date {}; latest has {}",
                *LAST_VIDEO_PUBLISHED_DATE.lock().await,
                date
            );
            if *LAST_VIDEO_PUBLISHED_DATE.lock().await < date {
                let bot = Bot::from_env().auto_send();
                info!(
                    "Big luca published a new video ({}): {}",
                    date,
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
                *LAST_VIDEO_PUBLISHED_DATE.lock().await = date;
            }
        }
        Ok(())
    }

    pub async fn notify_started() -> anyhow::Result<()> {
        let bot = Bot::from_env().auto_send();
        let message = AnswerBuilder::default()
            .text("😱😱😱 Il papi è tornato online 💣😎".to_string())
            .sticker(Stickers::got_it())
            .finalize();
        for chat in Self::subscribed_chats().await?.iter() {
            debug!("sending new video notify to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to send start notify to {}: {}", chat, err);
            }
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
