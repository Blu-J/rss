use std::{sync::Arc, time::Duration};

use color_eyre::Report;
use futures::{FutureExt, TryStreamExt};
use imbl::OrdMap;
use scraper::{Html, Selector};
use sqlx::{query_file, SqlitePool};
use tokio::{
    select,
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
};
use tracing::error;

use crate::{server::MyError, utils};

#[derive(Clone, Debug)]
pub struct Scraper {
    pool: SqlitePool,
    scrappers: Arc<Mutex<OrdMap<i64, Arc<Sender<()>>>>>,
}

impl Scraper {
    pub async fn new(pool: SqlitePool) -> Result<Self, MyError> {
        let scrapper = Self {
            pool: pool.clone(),
            scrappers: Default::default(),
        };
        let mut rows = query_file!("queries/sites_all.sql").fetch(&pool);
        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|x| MyError::Internal(x.into()))?
        {
            scrapper.scrape_site(row.id, row.user_id).await?;
        }
        Ok(scrapper)
    }

    pub async fn remove_scraper(&self, id: i64) {
        let mut scrappers = self.scrappers.lock().await;
        if let Some(cleanup) = scrappers.remove(&id) {
            cleanup.send(()).await.unwrap_or_default();
        };
    }

    pub async fn scrape_site(&self, site_id: i64, user_id: i64) -> Result<(), MyError> {
        let site = query_file!("queries/sites_by_id.sql", site_id, user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|x| MyError::CannotFind(x.into()))?;

        let (tx, mut rx) = channel(1);
        self.scrappers.lock().await.insert(site_id, Arc::new(tx));

        let url: reqwest::Url = site.url.parse().unwrap();
        let row_seconds = site.every_seconds as u64;
        let sleep_time = Duration::from_secs(row_seconds);
        let site_id = site.id;
        let site_title = site.site_title;
        let articles_sel = Selector::parse(&site.articles_sel).unwrap();
        let title_sel = Selector::parse(&site.title_sel).unwrap();
        let link_sel = Selector::parse(&site.link_sel).unwrap();
        let description_sel = site
            .description_sel
            .as_ref()
            .filter(|x| !x.is_empty())
            .map(|x| Selector::parse(x).unwrap());
        let image_sel = site
            .image_sel
            .as_ref()
            .filter(|x| !x.is_empty())
            .map(|x| Selector::parse(x).unwrap());
        let comments_sel = site
            .comments_sel
            .as_ref()
            .filter(|x| !x.is_empty())
            .map(|x| Selector::parse(x).unwrap());
        let pool = self.pool.clone();
        tokio::spawn(async move {
            let mut thread_sleep = Duration::from_secs_f32(0.5);
            tracing::info!(
                "Starting scraper for {} started with time of {}: {}",
                site_title,
                sleep_time.as_secs(),
                thread_sleep.as_secs_f32()
            );
            loop {
                select! {
                    _ = tokio::time::sleep(sleep_time) => {
                        tracing::info!("Should start");
                    },
                    _ = rx.recv() => {
                        tracing::info!("Scraper for site {} stopped for sleeping of {}", site_title,
                        sleep_time.as_secs());
                        break;
                    },
                };
                if thread_sleep.as_secs_f32() <= 1.0 {
                    tracing::info!("Changing Timer");
                    thread_sleep = sleep_time;
                }
                tracing::info!("Scraping site {}", site_title);
                let collections = match get_collections(
                    &url,
                    &articles_sel,
                    &link_sel,
                    &title_sel,
                    description_sel.as_ref(),
                    image_sel.as_ref(),
                    comments_sel.as_ref(),
                )
                .await
                {
                    Some(value) => value,
                    None => continue,
                };
                let mut transaction = pool.begin().await.unwrap();
                for ArticlePulled {
                    href,
                    title,
                    description,
                    image,
                    comments,
                } in collections
                {
                    let unix_time = utils::unix_time() as u32;
                    query_file!(
                        "queries/article_insert.sql",
                        site_id,
                        unix_time,
                        title,
                        href,
                        description,
                        image,
                        comments
                    )
                    .execute(&mut transaction)
                    .await
                    .map(|_| ())
                    .unwrap_or_else(|_| ());
                }
                transaction.commit().await.unwrap();
            }
        });
        Ok(())
    }
}

#[derive(Debug)]
pub struct ArticlePulled {
    pub href: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub comments: Option<String>,
}

pub async fn get_collections(
    url: &ammonia::Url,
    articles_sel: &Selector,
    link_sel: &Selector,
    title_sel: &Selector,
    description_sel: Option<&Selector>,
    image_sel: Option<&Selector>,
    comments_sel: Option<&Selector>,
) -> Option<Vec<ArticlePulled>> {
    let text = match reqwest::get(url.clone())
        .then(|x| async move { Ok::<_, Report>(x?.text().await?) })
        .await
    {
        Ok(x) => x,
        Err(e) => {
            error!("Error getting url: {:?}", e);
            return None;
        }
    };
    let collections = {
        let html = Html::parse_document(&text);
        html.select(articles_sel)
            .filter_map(|element| {
                let mut href = element
                    .select(link_sel)
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")?
                    .to_string();
                if !href.contains("http") {
                    let mut url = url.clone();
                    url.set_path(&href);
                    href = url.as_str().to_string();
                }

                let title = element.select(title_sel).next()?.text().next()?;

                let description: Option<String> = description_sel
                    .and_then(|sel| element.select(sel).next())
                    .and_then(|x| x.text().next())
                    .map(|x| x.to_string());
                let image: Option<String> = image_sel
                    .and_then(|sel| element.select(sel).next())
                    .and_then(|x| x.value().attr("src"))
                    .map(|x| x.to_string());
                let comments: Option<String> = comments_sel
                    .and_then(|sel| element.select(sel).next())
                    .and_then(|x| x.value().attr("href"))
                    .map(|x| x.to_string());

                Some(ArticlePulled {
                    href,
                    title: title.to_string(),
                    description,
                    image,
                    comments,
                })
            })
            .collect::<Vec<_>>()
    };
    Some(collections)
}

// #[tokio::test]
// async fn test_scraper() {
//     let clients = Clients::new(Settings::new().unwrap()).await.unwrap();
//     spawn_scraper(clients);
//     tokio::select! {
//         _ = tokio::time::sleep(Duration::from_secs(5)).boxed() => {
//         },
//     }
// }
