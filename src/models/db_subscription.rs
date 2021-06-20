use std::time::SystemTime;

use color_eyre::{eyre::bail, Result};
use rss::Channel;
use sqlx::{query, query_file_as, Executor, Sqlite};

use super::DbItem;

#[derive(Debug, Clone)]
pub struct DbSubscription {
    pub id: Option<i64>,
    pub unreads: Option<i64>,
    pub title: String,
    pub category: String,
    pub rss_feed: String,
}
impl DbSubscription {
    pub async fn get_items<'a>(&self) -> Result<Vec<DbItem>> {
        let subscription_id = match self.id {
            None => bail!("Expecting that the id is there"),
            Some(x) => x,
        };
        let content = reqwest::get(self.rss_feed.clone()).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;
        let items = channel
            .items
            .into_iter()
            .map(|item| DbItem {
                id: None,
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

    pub async fn fetch<'a>(
        id: i64,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Option<Self>> {
        let answer = query_file_as!(Self, "queries/subscription_fetch.sql", id)
            .fetch_optional(executor)
            .await?;
        Ok(answer)
    }
    pub async fn fetch_all<'a>(
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Vec<Self>> {
        let answer = query_file_as!(Self, "queries/subscription_fetch_all.sql")
            .fetch_all(executor)
            .await?;
        Ok(answer)
    }
    pub async fn insert<'a>(&self, executor: impl Executor<'a, Database = Sqlite>) -> Result<()> {
        query!(
            r#"INSERT INTO subscriptions ( category, title, rss_feed) VALUES (?, ?, ?) ON CONFLICT DO NOTHING"#,
            self.category,
            self.title,
            self.rss_feed
        )
        .execute(executor)
        .await?;
        Ok(())
    }
}
