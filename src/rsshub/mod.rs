//! # Rss hub
//!
//! Rss hub client

use feed_rs::parser as feed_parser;
use std::io::Cursor;

mod errors;

use crate::feed::Feed;
pub use errors::{RssHubError, RssHubResult};

/// RssHub feed client
pub struct RssHubClient {
    url: String,
}

impl RssHubClient {
    /// Instantiate a new `RssHubClient`
    pub fn new(url: impl ToString) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    /// Fetch youtube channel feed
    pub async fn fetch_instagram(&self, account: &str) -> RssHubResult<Feed> {
        let uri = format!("{}/instagram/user/{}", self.url, account);
        let body = self.fetch_feed(&uri).await?;
        trace!("Got body {}", body);
        self.parse_feed(body)
    }

    /// Fetch youtube channel feed
    async fn fetch_feed(&self, uri: &str) -> RssHubResult<String> {
        debug!("fetching feed {}", uri);
        reqwest::get(uri)
            .await
            .map_err(RssHubError::from)?
            .text()
            .await
            .map_err(RssHubError::from)
    }

    /// Parse feed
    fn parse_feed(&self, feed: String) -> RssHubResult<Feed> {
        debug!("parsing feed");
        let reader = Cursor::new(feed);
        feed_parser::parse(reader)
            .map(Feed::from)
            .map_err(RssHubError::from)
    }
}
