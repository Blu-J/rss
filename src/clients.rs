use color_eyre::eyre::Result;
use sqlx::sqlite::SqlitePool;

use crate::settings::Settings;

use self::scraper::Scraper;

pub mod scraper;

#[derive(Clone, Debug)]
pub struct Clients {
    pub pool: SqlitePool,
    pub settings: Settings,
    pub scraper: Scraper,
}

impl Clients {
    pub async fn new(settings: Settings) -> Result<Clients> {
        let pool = SqlitePool::connect(&settings.db_name).await?;
        let scraper = Scraper::new(pool.clone()).await?;
        Ok(Clients {
            pool,
            scraper,
            settings,
        })
    }
}
