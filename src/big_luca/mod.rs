//! # Big Luca
//!
//! This module implements the big luca bot

mod answer;
mod aphorism;
mod automatize;
mod commands;
mod config;
mod repository;
mod stickers;
mod youtube;

use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};
use url::Url;

use answer::{Answer, AnswerBuilder};
use aphorism::Aphorism;
use automatize::Automatizer;
use commands::Command;
pub use config::Config;
use once_cell::sync::OnceCell;
use stickers::Stickers;

pub static AUTOMATIZER: OnceCell<Automatizer> = OnceCell::new();

/// Big luca bot application
pub struct BigLuca {
    bot: AutoSend<Bot>,
}

impl BigLuca {
    /// Initialize big luca
    pub async fn init() -> anyhow::Result<Self> {
        // parse configuration
        if let Err(err) = Config::try_from_env() {
            return Err(err);
        }
        let automatizer = Automatizer::start()
            .await
            .map_err(|e| anyhow::anyhow!("failed to start automatizer: {}", e))?;
        if AUTOMATIZER.set(automatizer).is_err() {
            anyhow::bail!("failed to set automatizer");
        };
        let bot = Bot::from_env().auto_send();
        Ok(Self { bot })
    }

    /// Run big luca bot
    pub async fn run(self) -> anyhow::Result<()> {
        // setup hooks
        let port = Self::get_heroku_port()?;
        if let Some(port) = port {
            Self::run_on_heroku(self, port).await
        } else {
            Self::run_simple(self).await
        }
    }

    /// run bot with heroku webhooks
    async fn run_on_heroku(self, port: u16) -> anyhow::Result<()> {
        info!("running bot with heroku listener (PORT: {})", port);
        let addr = ([0, 0, 0, 0], port).into();
        let token = self.bot.inner().token();
        let host = std::env::var("HOST").map_err(|_| anyhow::anyhow!("HOST is not SET"))?;
        let url = Url::parse(&format!("https://{host}/webhooks/{token}")).unwrap();
        debug!("configuring listener {}...", url);
        let listener = webhooks::axum(self.bot.clone(), webhooks::Options::new(addr, url))
            .await
            .map_err(|e| anyhow::anyhow!("could not configure listener: {}", e))?;
        // start bot
        teloxide::commands_repl_with_listener(self.bot, Self::answer, listener, Command::ty())
            .await;
        Self::katanga_reminder().await;
        Ok(())
    }

    /// run bot without webhooks
    async fn run_simple(self) -> anyhow::Result<()> {
        info!("running bot without webhooks");
        teloxide::commands_repl(self.bot, Self::answer, Command::ty()).await;
        Self::katanga_reminder().await;
        Ok(())
    }

    /// Answer handler for bot
    async fn answer(
        bot: AutoSend<Bot>,
        message: Message,
        command: Command,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("got command {:?}", command);
        let answer = match command {
            Command::Help => Answer::simple_text(Command::descriptions()),
            Command::BigCorsi => Self::active_courses(),
            Command::BigKatanga => Self::subscribe_to_automatizer(&message.chat.id).await,
            Command::BigNews => Self::get_latest_videos().await,
            Command::BigPerla => Self::aphorism(),
            Command::BigPezzente => Self::unsubscribe_from_automatizer(&message.chat.id).await,
            Command::BigSito => Self::big_luca_website(),
            Command::BigVideo => Self::get_latest_video().await,
        };
        answer.send(&bot, message.chat.id).await
    }

    /// Send a random aphorism
    fn aphorism() -> Answer {
        AnswerBuilder::default()
            .text(Aphorism::get_random())
            .sticker(Stickers::random())
            .finalize()
    }

    /// Get latest videos from papi
    async fn get_latest_videos() -> Answer {
        match youtube::Youtube::get_latest_videos().await {
            Ok(feed) => {
                let mut message = String::new();
                for video in feed.videos() {
                    message.push_str(
                        format!(
                            "• {} 👉 {}\n",
                            video.title.as_deref().unwrap_or_default(),
                            video.url
                        )
                        .as_str(),
                    );
                }
                Answer::simple_text(message)
            }
            Err(err) => Self::error(err),
        }
    }

    /// Get latest video from papi
    async fn get_latest_video() -> Answer {
        match youtube::Youtube::get_latest_video().await {
            Ok(video) => Answer::simple_text(format!(
                "😱 {} 👉 {}",
                video.title.unwrap_or_default(),
                video.url
            )),
            Err(err) => Self::error(err),
        }
    }

    /// Get big luca website
    fn big_luca_website() -> Answer {
        Answer::simple_text(
            "Vai sul sito della Big Luca International 👉 https://www.biglucainternational.com/it",
        )
    }

    /// Get latest active courses
    fn active_courses() -> Answer {
        AnswerBuilder::default()
        .text(
            r#"È uscito il nuovo libro del Papi "Alzati (tardi) e lucra!", solo fino al 26 agosto 👉 https://bit.ly/alzatielucra"#,
        ).finalize()
    }

    /// Subscribe chat to the automatizer
    async fn subscribe_to_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().subscribe(chat_id).await {
            Ok(_) => AnswerBuilder::default()
            .text("sei ora iscritto alla piattaforma Katanga! 🚀🚀🚀 Da ora riceverai tutte le perle del papi e i suoi ultimi aggiornamenti automaticamente su questa chat! 😱")
            .sticker(Stickers::got_it())
            .finalize(),
            Err(err) => Self::error(err),
        }
    }

    async fn unsubscribe_from_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().unsubscribe(chat_id).await {
            Ok(()) => AnswerBuilder::default()
                .text("ti sei disinscritto dalla piattaforma Katanga 😡, ora torna pure dai tuoi amici sfigati a vendere ai poveri. 😜")
                .sticker(Stickers::grrr())
                .finalize(),
            Err(err) => Self::error(err),
        }
    }

    /// Send a reminder to all the subscribed chats, to remind them to resubscribe
    async fn katanga_reminder() {
        info!("bot is shutting down; sending katanga reminder");
        let bot = Bot::from_env().auto_send();
        let message = AnswerBuilder::default()
            .text("il Papi bot si sta riavviando 😱😨, quando torna dal volo col privatino ti avvisa 😎")
            .sticker(Stickers::oh_no())
            .finalize();
        let chats = match automatize::Automatizer::subscribed_chats().await {
            Ok(c) => c,
            Err(err) => {
                error!("failed to get chats: {}", err);
                return;
            }
        };
        for chat in chats.iter() {
            debug!("sending katanga reminder to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to katang reminder to {}: {}", chat, err);
            }
        }
    }

    /// The answer to return in case of an error
    fn error(err: impl ToString) -> Answer {
        AnswerBuilder::default()
            .text(err)
            .sticker(Stickers::despair())
            .finalize()
    }

    // get heroku port
    fn get_heroku_port() -> anyhow::Result<Option<u16>> {
        match std::env::var("PORT").map(|x| x.parse()) {
            Err(_) => Ok(None),
            Ok(Ok(p)) => Ok(Some(p)),
            Ok(Err(e)) => anyhow::bail!("could not parse PORT environment variable: {}", e),
        }
    }
}
