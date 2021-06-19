use color_eyre::eyre::Result;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct Clients {
    pub pool: SqlitePool,
}

impl Clients {
    pub async fn new(db_name: &str) -> Result<Clients> {
        Ok(Clients {
            pool: SqlitePool::connect(db_name).await?,
        })
    }
}
