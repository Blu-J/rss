use color_eyre::Result;
use sqlx::{query_file_as, Executor, Sqlite};
use tracing::instrument;

use super::{Subscription, UserId};

#[derive(Debug, Clone)]
pub struct UserSubscription {
    pub id: i64,
    pub title: String,
    pub category: String,
    pub rss_feed: String,
}
impl UserSubscription {
    #[instrument(skip(executor))]
    pub async fn fetch<'a>(
        user_id: &UserId,
        id: i64,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Self> {
        let answer = query_file_as!(Self, "queries/user_subscription_fetch.sql", user_id, id,)
            .fetch_one(executor)
            .await?;
        Ok(answer)
    }
    #[instrument(skip(executor))]
    pub async fn fetch_all<'a>(
        user_id: &UserId,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Vec<Self>> {
        let answer = query_file_as!(Self, "queries/user_subscription_fetch_all.sql", user_id)
            .fetch_all(executor)
            .await?;
        Ok(answer)
    }
    #[instrument(skip(executor))]
    pub async fn insert<'a>(
        category: &str,
        title: &str,
        subscription: &Subscription,
        user_id: &UserId,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Self> {
        let answer = query_file_as!(
            Self,
            "queries/user_subscription_insert.sql",
            category,
            title,
            user_id,
            subscription.id,
            user_id,
            subscription.id,
        )
        .fetch_one(executor)
        .await?;
        Ok(answer)
    }
}
