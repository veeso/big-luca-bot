//! # Youtube
//!
//! A youtube feed client

use feed_rs::parser as feed_parser;
use std::io::Cursor;

mod errors;
mod feed;

pub use errors::{YoutubeError, YoutubeResult};
pub use feed::{Feed, Video};

/// Youtube feed client
pub struct YoutubeClient {
    channel_id: String,
}

impl YoutubeClient {
    /// Instantiate a new `YoutubeClient`
    pub fn new(channel_id: impl ToString) -> Self {
        Self {
            channel_id: channel_id.to_string(),
        }
    }

    /// Fetch youtube channel feed
    pub async fn fetch(&self) -> YoutubeResult<Feed> {
        let body = self.fetch_feed().await?;
        trace!("Got body {}", body);
        self.parse_feed(body)
    }

    /// Fetch youtube channel feed
    async fn fetch_feed(&self) -> YoutubeResult<String> {
        let uri = format!(
            "https://www.youtube.com/feeds/videos.xml?channel_id={}",
            self.channel_id
        );
        debug!("fetching youtube channel {}", uri);
        reqwest::get(uri)
            .await
            .map_err(YoutubeError::from)?
            .text()
            .await
            .map_err(YoutubeError::from)
    }

    /// Parse feed
    fn parse_feed(&self, feed: String) -> YoutubeResult<Feed> {
        debug!("parsing feed");
        let reader = Cursor::new(feed);
        feed_parser::parse(reader)
            .map(Feed::from)
            .map_err(YoutubeError::from)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn should_fetch_feed() {
        let client = YoutubeClient::new("UCTpU7OQg9QVsqayEYXTL1LQ");
        assert!(client.fetch_feed().await.is_ok());
    }

    #[tokio::test]
    async fn should_parse_feed() {
        let client = YoutubeClient::new("UCTpU7OQg9QVsqayEYXTL1LQ");
        let feed = client.fetch().await.unwrap();
        assert!(feed.videos().next().is_some());
    }
}
