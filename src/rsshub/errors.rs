//! # Errors
//!
//! RssHub client errors

use feed_rs::parser::ParseFeedError;
use thiserror::Error;

pub type RssHubResult<T> = Result<T, RssHubError>;

/// An error returned by the RssHub Client
#[derive(Debug, Error)]
pub enum RssHubError {
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    #[error("Feed parser error: {0}")]
    FeedParserError(ParseFeedError),
}

impl From<reqwest::Error> for RssHubError {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(e)
    }
}

impl From<ParseFeedError> for RssHubError {
    fn from(e: ParseFeedError) -> Self {
        Self::FeedParserError(e)
    }
}
