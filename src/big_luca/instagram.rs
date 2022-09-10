//! # Instagram
//!
//! Big luca bot instagram scraper

use std::time::SystemTime;

use instagram_scraper_rs::{InstagramScraper, Post};

use super::Config;

pub struct InstagramService;

const BIGLUCA_ACCOUNT_ID: &str = "bigluca.marketing";

impl InstagramService {
    /// Get newest (latest) post from instagram
    pub async fn get_latest_post() -> anyhow::Result<Post> {
        let config = Config::try_from_env()?;
        debug!("creating instagram scraper");
        let mut scraper = InstagramScraper::default()
            .authenticate_with_login(config.instagram_username, config.instagram_password);
        scraper.login().await?;
        let user_id = Self::get_user_id(&mut scraper).await?;
        let posts = Self::get_posts(&mut scraper, &user_id, 1).await?;
        scraper.logout().await?;
        if let Some(post) = posts.get(0) {
            Ok(post.clone())
        } else {
            anyhow::bail!("Non ho trovato nessun post sull'instagram del papi");
        }
    }

    /// Get oldest unseen post from instagram
    pub async fn get_oldest_unseen_post(
        last_post_pubdate: SystemTime,
    ) -> anyhow::Result<Option<Post>> {
        let config = Config::try_from_env()?;
        debug!("creating instagram scraper");
        let mut scraper = InstagramScraper::default()
            .authenticate_with_login(config.instagram_username, config.instagram_password);
        scraper.login().await?;
        let user_id = Self::get_user_id(&mut scraper).await?;
        let mut posts = Self::get_posts(&mut scraper, &user_id, 50).await?;
        scraper.logout().await?;
        posts.sort_by_key(|x| x.taken_at_timestamp);
        for entry in posts.into_iter() {
            if entry.taken_at_timestamp > last_post_pubdate {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    /// Get posts
    async fn get_posts(
        scraper: &mut InstagramScraper,
        user_id: &str,
        count: usize,
    ) -> anyhow::Result<Vec<Post>> {
        debug!("getting posts for {}", user_id);
        let posts = scraper.scrape_posts(user_id, count).await?;
        Ok(posts)
    }

    /// Get user id from instagram for bigluca
    async fn get_user_id(scraper: &mut InstagramScraper) -> anyhow::Result<String> {
        let user = scraper.scrape_userinfo(BIGLUCA_ACCOUNT_ID).await?;
        Ok(user.id)
    }
}
