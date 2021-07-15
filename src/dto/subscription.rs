use std::time::SystemTime;

use chrono::DateTime;
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
                pub_date: dbg!(dbg!(item.pub_date).as_ref().and_then(|x| parse_date(x)))
                    .unwrap_or_else(SystemTime::now)
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

fn parse_date(date: &str) -> Option<SystemTime> {
    if let Some(date) = DateTime::parse_from_rfc2822(date).ok() {
        return Some(date.into());
    }
    if let Some(date) = DateTime::parse_from_rfc3339(date).ok() {
        return Some(date.into());
    }
    None
}

#[cfg(test)]
mod tests {
    use replay_mocker::{mocks::ReplayMock, MockServer};

    use super::*;
    #[actix_web::rt::test]
    async fn test_get_items() {
        let mock = MockServer::new()
            .await
            .mock(ReplayMock::from_file("./test/artifacts/sword_feed.json"))
            .await;
        let subscription = Subscription {
            id: 0,
            rss_feed: format!("http://{}/", mock.address),
        };

        let items = subscription.get_items().await.unwrap();
        assert_eq!(items.len(), 10);
        assert_eq!(&items[0].title, &"DXIII - Battlefruit");
        assert_eq!(&items[0].link, &"https://swordscomic.com/comic/DXIII/");
        assert!(items[0].pub_date > 1626309745);
        assert_eq!(
            items[0].author.clone().unwrap_or_default(),
            "Matthew Wills".to_string()
        );
        assert_eq!(
            items[0].description.clone().unwrap_or_default(),
            "\n      <style>\n        .tag {\n          background-color: var(--tag-background-color);\n          margin: 1px 2px;\n          padding: 4px;\n          padding-left: 24px;\n          height: 16px;\n          line-height: 16px;\n          font-size: 12px;\n          text-align: center;\n          text-decoration: none;\n          display: inline-block;\n          background-image: var(--default-tag-icon);\n          background-size: 16px 16px;\n          background-position: 4px 4px;\n          background-repeat: no-repeat;\n        }\n      </style>\n      <img src=\"https://swordscomic.com/media/Swords513T.png\"></img>\n      <p><strong>Sea Captain:</strong> Draw your weapon!</p>\n\n<blockquote>\n  <p>The Captain pulls out his sword with a SHING! sound</p>\n</blockquote>\n\n<p><strong>Jolly Pirate:</strong> En garde!</p>\n\n<blockquote>\n  <p>The Jolly Pirate pulls out a banana with a WHIP! sound</p>\n</blockquote>\n\n<p><strong>Jolly Pirate:</strong> Whoops! Ha ha ha!</p>\n\n<blockquote>\n  <p>The Jolly Pirate peels the banana open with a PEEL! sound to reveal a sword blade inside</p>\n</blockquote>\n\n<p><strong>Jolly Pirate:</strong> En garde!</p>\n\n    ".to_string()
        );
        assert_eq!(
            items[0].contents.clone().unwrap_or_default(),
            "".to_string()
        );
    }
}
