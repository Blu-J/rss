use color_eyre::eyre::Result;
use sqlx::{query_as, query_file_as, Executor, Sqlite};
use tracing::instrument;

use super::UserId;
use crate::server::from_requests::user_preferences::FilterItems;
#[derive(Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub subscription_id: i64,
    pub title: String,
    pub link: String,
    pub pub_date: i64,
    pub author: Option<String>,
    pub description: Option<String>,
    pub contents: Option<String>,
    pub comments: Option<String>,
}

impl Item {
    #[instrument(skip(executor))]
    pub async fn fetch<'a>(
        id: i64,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Option<Self>> {
        let answer = query_as!(
            Self,
            r#"SELECT id, subscription_id, title, pub_date,  link, author, description,contents, comments FROM items where id = ?"#,
            id
        )
        .fetch_optional(executor)
        .await?;
        Ok(answer)
    }
    #[instrument(skip(executor))]
    pub async fn fetch_all_not_read<'a>(
        user_id: &UserId,
        filter_items: &FilterItems,
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Vec<Self>> {
        let (id, title) = filter_items.as_items();
        let answer = query_file_as!(Self, "queries/user_item_fetch_all.sql", user_id, id, title)
            .fetch_all(executor)
            .await?;
        Ok(answer)
    }
}
