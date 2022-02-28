use scraper::{Html, Selector};
use std::{
    ops::Sub,
    time::{Duration, SystemTime},
};
use tracing_subscriber::EnvFilter;

use crate::{clients::Clients, server::spawn_server};
use actix_web::rt::{
    signal::{
        ctrl_c,
        unix::{signal, SignalKind},
    },
    spawn,
    time::{self, timeout},
};

use chrono::Utc;
use color_eyre::{owo_colors::Color, Report};
use futures::{select, stream, FutureExt, StreamExt};

use futures::TryStreamExt;
use settings::Settings;
use sqlx::query_file;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, warn};

pub mod clients;
// pub mod dto;
pub mod server;
pub mod session;
pub mod settings;

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    install_tracing()?;
    info!("Hello, world!");
    let clients = Clients::new(Settings::new().unwrap()).await?;

    let server = spawn_server(clients.clone());
    let _ = spawn_scraper(clients.clone());
    server.await?;
    Ok(())
}
fn install_tracing() -> color_eyre::Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;
    color_eyre::install()?;

    let fmt_layer = fmt::layer().with_target(false);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("rss=info,warn"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}

fn spawn_scraper(clients: Clients) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        info!("Starting scraper");
        let mut rows = query_file!("queries/sites_all.sql").fetch(&clients.pool);
        while let Some(row) = rows.try_next().await.unwrap() {
            let url: reqwest::Url = row.url.parse().unwrap();
            let row_seconds = row.every_seconds as u64;
            let sleep_time = Duration::from_secs(row_seconds);
            let site_id = row.id;
            let articles_sel = Selector::parse(&row.articles_sel).unwrap();
            let title_sel = Selector::parse(&row.title_sel).unwrap();
            let link_sel = Selector::parse(&row.link_sel).unwrap();
            let description_sel = row
                .description_sel
                .as_ref()
                .map(|x| Selector::parse(x).unwrap());
            let image_sel = row.image_sel.as_ref().map(|x| Selector::parse(x).unwrap());
            let comments_sel = row
                .comments_sel
                .as_ref()
                .map(|x| Selector::parse(x).unwrap());
            let clients = clients.clone();
            tokio::spawn(async move {
                let mut first = true;
                loop {
                    if !first {
                        tokio::time::sleep(sleep_time).await
                    }
                    first = false;
                    let text = match reqwest::get(url.clone())
                        .then(|x| async move { Ok::<_, Report>(x?.text().await?) })
                        .await
                    {
                        Ok(x) => x,
                        Err(e) => {
                            error!("Error getting url: {:?}", e);
                            continue;
                        }
                    };
                    let collections = {
                        let html = Html::parse_document(&text);
                        html.select(&articles_sel)
                            .filter_map(|element| {
                                let mut href = element
                                    .select(&link_sel)
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

                                let title = element.select(&title_sel).next()?.text().next()?;

                                let description: Option<String> = description_sel
                                    .as_ref()
                                    .and_then(|sel| element.select(sel).next())
                                    .and_then(|x| x.text().next())
                                    .map(|x| x.to_string());
                                let image: Option<String> = image_sel
                                    .as_ref()
                                    .and_then(|sel| element.select(sel).next())
                                    .and_then(|x| x.value().attr("src"))
                                    .map(|x| x.to_string());
                                let comments: Option<String> = comments_sel
                                    .as_ref()
                                    .and_then(|sel| element.select(sel).next())
                                    .and_then(|x| x.value().attr("href"))
                                    .map(|x| x.to_string());

                                Some((href, title.to_string(), description, image, comments))
                            })
                            .collect::<Vec<_>>()
                    };
                    let mut transaction = clients.pool.begin().await.unwrap();
                    for (href, title, description, image, comments) in collections {
                        let unix_time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs_f64();
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
        }
    })
}

#[tokio::test]
async fn test_scraper() {
    let clients = Clients::new(Settings::new().unwrap()).await.unwrap();
    spawn_scraper(clients);
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(5)).boxed() => {
        },
    }
}
