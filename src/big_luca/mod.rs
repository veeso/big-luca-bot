//! # Big Luca
//!
//! This module implements the big luca bot

mod answer;
mod aphorism;
mod commands;
mod stickers;
mod youtube;

use teloxide::{prelude::*, utils::command::BotCommands};

use answer::{Answer, AnswerBuilder};
use aphorism::Aphorism;
use commands::Command;
use stickers::Stickers;

/// Big luca bot application
pub struct BigLuca {
    bot: AutoSend<Bot>,
}

impl BigLuca {
    /// Initialize big luca
    pub async fn init() -> anyhow::Result<Self> {
        if std::env::var("TELOXIDE_TOKEN").is_err() {
            anyhow::bail!("TELOXIDE_TOKEN is NOT set. You must set this variable in the environment with your bot token API")
        }
        let bot = Bot::from_env().auto_send();
        Ok(Self { bot })
    }

    /// Run big luca bot
    pub async fn run(self) -> anyhow::Result<()> {
        teloxide::commands_repl(self.bot, Self::answer, Command::ty()).await;
        Ok(())
    }

    /// Answer handler for bot
    async fn answer(
        bot: AutoSend<Bot>,
        message: Message,
        command: Command,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let answer = match command {
            Command::Help => Answer::simple_text(Command::descriptions()),
            Command::BigCorsi => Self::active_courses(),
            Command::BigNews => Self::get_latest_videos().await,
            Command::BigPerla => Self::aphorism(),
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
                            "• {} ({})\n",
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
                "{} ({})",
                video.title.unwrap_or_default(),
                video.url
            )),
            Err(err) => Self::error(err),
        }
    }

    /// Get big luca website
    fn big_luca_website() -> Answer {
        Answer::simple_text(
            "Vai sul sito della Big Luca International https://www.biglucainternational.com/it",
        )
    }

    /// Get latest active courses
    fn active_courses() -> Answer {
        AnswerBuilder::default()
        .text(
            r#"È uscito il nuovo libro del Papi "Alzati (tardi) e lucra!", solo fino al 26 agosto 👉 https://bit.ly/alzatielucra"#,
        ).finalize()
    }

    /// The answer to return in case of an error
    fn error(err: impl ToString) -> Answer {
        AnswerBuilder::default()
            .text(err)
            .sticker(Stickers::despair())
            .finalize()
    }
}
