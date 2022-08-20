//! # Automatize
//!
//! A module to automatize messages

use super::{AnswerBuilder, Aphorism, Stickers};

use futures::lock::Mutex;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

type AutomatizerResult<T> = Result<T, AutomatizerError>;

lazy_static! {
    static ref AUTOMATIZER: Option<Mutex<Automatizer>> = None;
    static ref SUBSCRIBED_CHATS: Mutex<Vec<ChatId>> = Mutex::new(Vec::new());
}

/// Automatizer error
#[derive(Debug, Error)]
pub enum AutomatizerError {
    #[error("chat {0} is not currently subscribed to the automatizer")]
    ChatIsNotSubscribed(ChatId),
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
    pub async fn subscribe(&self, chat: &ChatId) {
        info!("subscribed {} to the automatizer", chat);
        SUBSCRIBED_CHATS.lock().await.push(*chat);
    }

    /// Unsubscribe chat from automatizer. If the chat is not currently subscribed, return error
    pub async fn unsubscribe(&self, chat: &ChatId) -> AutomatizerResult<()> {
        let index = SUBSCRIBED_CHATS.lock().await.iter().position(|x| x == chat);
        if let Some(index) = index {
            SUBSCRIBED_CHATS.lock().await.remove(index);
            info!("unsubscribed {} to the automatizer", chat);
            Ok(())
        } else {
            warn!("failed to unsubscribe {}; it's not subscribed", chat);
            Err(AutomatizerError::ChatIsNotSubscribed(*chat))
        }
    }

    /// Setup cron scheduler
    async fn setup_cron_scheduler() -> AutomatizerResult<JobScheduler> {
        let sched = JobScheduler::new().await?;
        let perla_job = Job::new_async("1/30 * * * * *", |_, _| {
            Box::pin(async move {
                info!("running perla-job");
                Self::send_perla().await;
            })
        })?;
        sched.add(perla_job).await?;
        sched
            .start()
            .await
            .map(|_| sched)
            .map_err(AutomatizerError::from)
    }

    /// Send perla
    async fn send_perla() {
        let bot = Bot::from_env().auto_send();
        let message = AnswerBuilder::default()
            .text(Aphorism::get_random())
            .sticker(Stickers::random())
            .finalize();
        for chat in SUBSCRIBED_CHATS.lock().await.iter() {
            debug!("sending scheduled aphorism to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to send scheduled aphorism to {}: {}", chat, err);
            }
        }
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
