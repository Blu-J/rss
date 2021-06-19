use color_eyre::eyre::Result;
use sqlx::{query, query_as, Executor, Sqlite};
#[derive(Debug, Clone)]
pub struct DbItem {
    pub id: Option<i64>,
    pub subscription_id: i64,
    pub title: String,
    pub link: String,
    pub pub_date: i64,
    pub author: Option<String>,
    pub description: Option<String>,
    pub comments: Option<String>,
}

impl DbItem {
    pub async fn fetch<'a>(
        executor: impl Executor<'a, Database = Sqlite>,
        id: &str,
    ) -> Result<Option<Self>> {
        let answer = query_as!(
            Self,
            r#"SELECT  id as "id?", subscription_id, title, pub_date,  link, author, description, comments FROM items where id = ?"#,
            id
        )
        .fetch_optional(executor)
        .await?;
        Ok(answer)
    }
    pub async fn insert<'a>(&self, executor: impl Executor<'a, Database = Sqlite>) -> Result<()> {
        query!("INSERT INTO items (id, subscription_id, title, link, pub_date, author, description, comments) VALUES (?, ?, ?, ?, ?, ?,?,?) ON CONFLICT DO NOTHING", self.id, self.subscription_id, self.title, self.link, self.pub_date, self.author, self.description, self.comments)
        .execute(executor)
        .await?;
        Ok(())
    }
    pub async fn fetch_all<'a>(
        executor: impl Executor<'a, Database = Sqlite>,
    ) -> Result<Vec<Self>> {
        let answer = query_as!(
            Self,
            r#"SELECT id as "id?", subscription_id, title, pub_date,  link, author, description, comments
            FROM items"#,
        )
        .fetch_all(executor)
        .await?;
        Ok(answer)
    }
}
