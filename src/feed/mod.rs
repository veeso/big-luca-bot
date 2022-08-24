//! # Feed
//!
//! This module exposes the types for the feed

use crate::utils::str as str_helpers;

use chrono::{DateTime, Utc};
use feed_rs::model::{Entry as RssEntry, Feed as RssFeed};
use std::slice::Iter;

/// Contains, for a feed source, the list of entries fetched from remote
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feed {
    pub(crate) entries: Vec<Entry>,
}

/// identifies a single article in the feed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub summary: String,
    pub url: String,
    pub date: Option<DateTime<Utc>>,
}

impl Feed {
    /// Get an iterator over entries
    pub fn entries(&self) -> Iter<'_, Entry> {
        self.entries.iter()
    }
}

// -- converter

impl From<RssFeed> for Feed {
    fn from(feed: RssFeed) -> Self {
        Self {
            entries: feed.entries.into_iter().map(Entry::from).collect(),
        }
    }
}

impl From<RssEntry> for Entry {
    fn from(entry: RssEntry) -> Self {
        let content_or_summary = content_or_summary(&entry);
        Self {
            title: entry
                .title
                .map(|x| str_helpers::strip_html(x.content.as_str())),
            authors: entry.authors.into_iter().map(|x| x.name).collect(),
            summary: content_or_summary,
            url: entry
                .links
                .get(0)
                .map(|x| x.href.clone())
                .unwrap_or(entry.id),
            date: entry.published.or(entry.updated).map(DateTime::<Utc>::from),
        }
    }
}

/// This function returns content if any, otherwise the summary of the article.
/// The reason is that content is USUALLY the entire article, BUT sometimes is not filled, so summary is preferred in these cases
fn content_or_summary(entry: &RssEntry) -> String {
    let content = entry
        .content
        .as_ref()
        .and_then(|x| x.body.as_ref().map(|x| str_helpers::strip_html(x)))
        .unwrap_or_default();
    if content.trim_matches('\n').trim().is_empty() {
        // get summary instead
        entry
            .summary
            .as_ref()
            .map(|x| str_helpers::strip_html(x.content.as_str()))
            .unwrap_or_default()
    } else {
        content
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use feed_rs::model::FeedType;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_feed_attributes() {
        let feed = Feed {
            entries: Vec::default(),
        };
        assert!(feed.entries.is_empty());
    }

    #[test]
    fn should_convert_entry_into_article() {
        let entry = RssEntry::default();
        let article = Entry::from(entry);
        assert!(article.authors.is_empty());
        assert_eq!(article.date, None);
        assert_eq!(article.summary, String::new());
        assert_eq!(article.title, None);
        assert_eq!(article.url, String::new());
    }

    #[test]
    fn should_convert_rssfeed_into_feed() {
        let feed = RssFeed {
            feed_type: FeedType::Atom,
            id: String::from("pippo"),
            contributors: Vec::new(),
            title: None,
            updated: None,
            authors: Vec::new(),
            description: None,
            links: Vec::new(),
            categories: Vec::new(),
            generator: None,
            icon: None,
            language: None,
            logo: None,
            published: None,
            rating: None,
            rights: None,
            ttl: None,
            entries: vec![RssEntry::default(), RssEntry::default()],
        };
        let feed = Feed::from(feed);
        assert_eq!(feed.entries.len(), 2);
    }
}
