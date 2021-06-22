use std::time::SystemTime;

use color_eyre::Result;
use rss::Channel;
use sqlx::{query_as, query_file_as, Executor, Sqlite};
use tracing::instrument;

use super::ItemInsert;

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: i64,
    pub rss_feed: String,
}
impl Subscription {
    #[instrument]
    pub async fn get_items<'a>(&self) -> Result<Vec<ItemInsert>> {
        let subscription_id = self.id;
        let content = reqwest::get(self.rss_feed.clone()).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;
        let items = channel
            .items
            .into_iter()
            .map(|item| ItemInsert {
                subscription_id,
                title: item.title.unwrap_or_default(),
                link: item.link.unwrap_or_default(),
                pub_date: item
                    .pub_date
                    .clone()
                    .and_then(|x| httpdate::parse_http_date(&x).ok())
                    .unwrap_or_else(|| SystemTime::now())
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .unwrap_or_default()
                    .as_secs() as i64,
                author: item.author,
                description: item.description,
                comments: item.comments,
                contents: item.content,
            })
            .collect();
        Ok(items)
    }

    #[instrument(skip(executor))]
    pub async fn fetch<'a>(executor: impl Executor<'a, Database = Sqlite>) -> Result<Self> {
        let answer = query_file_as!(Self, "queries/subscription_fetch_all.sql")
            .fetch_one(executor)
            .await?;
        Ok(answer)
    }
    #[instrument(skip(executor))]
    pub async fn fetch_all<'a>(
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Vec<Self>> {
        let answer = query_file_as!(Self, "queries/subscription_fetch_all.sql")
            .fetch_all(executor)
            .await?;
        Ok(answer)
    }
    pub async fn insert<'a>(
        rss_feed: &str,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Self> {
        let answer = query_as!(
            Self,
            r#"INSERT INTO subscriptions (rss_feed) VALUES ($1) ON CONFLICT DO NOTHING;
            SELECT id, rss_feed FROM subscriptions WHERE rss_feed = $1;"#,
            rss_feed,
            rss_feed
        )
        .fetch_one(executor)
        .await?;
        Ok(answer)
    }
}
