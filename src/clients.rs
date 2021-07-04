use color_eyre::eyre::Result;
use sqlx::sqlite::SqlitePool;

use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct Clients {
    pub pool: SqlitePool,
    pub settings: Settings,
}

impl Clients {
    pub async fn new(settings: Settings) -> Result<Clients> {
        Ok(Clients {
            pool: SqlitePool::connect(&settings.db_name).await?,
            settings,
        })
    }
}
