use color_eyre::eyre::Result;
use sqlx::{query, Executor, Sqlite};
use tracing::instrument;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemInsert {
    pub subscription_id: i64,
    pub title: String,
    pub link: String,
    pub pub_date: i64,
    pub author: Option<String>,
    pub description: Option<String>,
    pub contents: Option<String>,
    pub comments: Option<String>,
}

impl ItemInsert {
    #[instrument(skip(executor))]
    pub async fn insert<'a>(&self, executor: impl Executor<'a, Database = Sqlite>) -> Result<()> {
        query!("INSERT INTO items (subscription_id, title, link, pub_date, author, description, comments) VALUES (?, ?, ?, ?, ?,?,?) ON CONFLICT DO NOTHING",  self.subscription_id, self.title, self.link, self.pub_date, self.author, self.description, self.comments)
        .execute(executor)
        .await?;
        Ok(())
    }
}
