//! # Answer
//!
//! This module cares of providing answer script types and sending messages

use std::str::FromStr;

use teloxide::prelude::*;
use teloxide::types::InputFile;
use url::Url;

/// A helper to build composed answers
#[derive(Default)]
pub struct AnswerBuilder {
    answer: Answer,
}

impl AnswerBuilder {
    /// Add text to script
    pub fn text(mut self, text: impl ToString) -> Self {
        self.answer.script.push(Media::Text(text.to_string()));
        self
    }

    /// Add sticker to script
    pub fn sticker(mut self, sticker: InputFile) -> Self {
        self.answer.script.push(Media::Sticker(sticker));
        self
    }

    /// Add image to script
    pub fn image<S: AsRef<str>>(mut self, url: S) -> Self {
        if let Ok(url) = Url::from_str(url.as_ref()) {
            self.answer.script.push(Media::Image(InputFile::url(url)));
        }
        self
    }

    /// Finalize builder
    pub fn finalize(self) -> Answer {
        self.answer
    }
}

/// The answer to send to the chat
#[derive(Default, Clone)]
pub struct Answer {
    script: Vec<Media>,
}

#[derive(Clone)]
/// A media in the chat
enum Media {
    Text(String),
    Image(InputFile),
    Sticker(InputFile),
}

impl Answer {
    /// Build a simple one text answer
    pub fn simple_text(text: impl ToString) -> Self {
        Self {
            script: vec![Media::Text(text.to_string())],
        }
    }

    /// Send answer
    pub async fn send(self, bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        for message in self.script.into_iter() {
            match message {
                Media::Image(image) => Self::send_image(bot, chat_id, image).await?,
                Media::Sticker(sticker) => Self::send_sticker(bot, chat_id, sticker).await?,
                Media::Text(text) => Self::send_text(bot, chat_id, text).await?,
            }
        }
        Ok(())
    }

    /// Write text to chat
    async fn send_text(bot: &Bot, chat_id: ChatId, message: String) -> ResponseResult<()> {
        bot.send_message(chat_id, message).await.map(|_| ())
    }

    /// Send image to chat
    async fn send_image(bot: &Bot, chat_id: ChatId, image: InputFile) -> ResponseResult<()> {
        bot.send_photo(chat_id, image).await.map(|_| ())
    }

    /// Send sticker
    async fn send_sticker(bot: &Bot, chat_id: ChatId, sticker: InputFile) -> ResponseResult<()> {
        bot.send_sticker(chat_id, sticker).await.map(|_| ())
    }
}
