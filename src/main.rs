use std::{collections::HashMap, ops::Sub, time::Duration};

use crate::{
    clients::Clients,
    models::{DbItem, DbSubscription},
};
use actix_web::{
    body::Body,
    error, get,
    http::StatusCode,
    middleware, post,
    rt::{
        signal::{
            ctrl_c,
            unix::{signal, SignalKind},
        },
        spawn,
        time::{self, timeout},
    },
    web, App, HttpResponse, HttpServer,
};
use askama::Template;
use chrono::Utc;
use color_eyre::Report;
use futures::{select, FutureExt};
use lazy_static::lazy_static;
use rss::Channel;
use serde::Deserialize;
use settings::Settings;
use tracing::{info, instrument, warn};
use uuid::Uuid;

pub mod clients;
pub mod models;
pub mod settings;

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().unwrap();
}

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    install_tracing()?;
    info!("Hello, world!");
    let clients = Clients::new("./data.db").await?;

    let server = {
        let clients = clients.clone();
        spawn(async move {
            HttpServer::new(move || {
                App::new()
                    .data(clients.clone())
                    .wrap(middleware::Compress::default())
                    .service(new_subscription)
                    .service(actix_files::Files::new("/static", "./static").show_files_listing())
                    .service(get_index)
            })
            .bind("0.0.0.0:8080")
            .expect("starting server")
            .run()
            .await
            .expect("running server");
        })
    };
    let task2 = spawn(async move {
        async {
            let mut items: Vec<DbItem> = Vec::with_capacity(10);
            let mut sigup = signal(SignalKind::hangup())?;
            loop {
                select! {
                _ = timeout(Duration::from_secs(30), async {
                        items.truncate(0);
                        let start = Utc::now();
                        for subscription in DbSubscription::fetch_all(&clients.pool).await? {
                            for item in subscription.get_items().await? {
                                items.push(item);
                            }
                        }
                        let mut transaction = clients.pool.begin().await?;
                        for item in items.iter() {
                            item.insert(&mut transaction).await?;
                        }
                        transaction.commit().await?;
                        let duration = Utc::now().sub(start);
                        info!("Time to insert {} items: {}", items.len(), duration);
                        time::sleep(Duration::from_secs(10)).await;

                        Ok::<_, Report>(())
                    }).fuse() => (),
                    _ = ctrl_c().fuse() => break,
                    _ = sigup.recv().fuse() => break,
                }
            }

            Ok::<_, Report>(())
        }
        .await
        .expect("Running in task 2");
    });
    server.await?;
    task2.await?;
    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
struct SubscriptionForm {
    category: String,
    title: String,
    url: String,
}

#[post("/subscriptions")]
#[instrument(skip(clients))]
async fn new_subscription(
    path: web::Form<SubscriptionForm>,
    clients: web::Data<Clients>,
) -> Result<HttpResponse, MyError> {
    info!("Starting a new subscription {:?}", path);
    let SubscriptionForm {
        category,
        title,
        url,
    } = path.into_inner();

    let content = reqwest::get(url.clone())
        .await
        .map_err(|e| MyError::BadParam("url".into(), format!("{:?}", e)))?
        .bytes()
        .await
        .map_err(|e| MyError::BadParam("url".into(), format!("{:?}", e)))?;
    let channel = Channel::read_from(&content[..])
        .map_err(|x| MyError::InvalidSubscription(url.clone(), x.to_string()))?;

    let subscription = DbSubscription {
        id: None,
        title,
        rss_feed: format!("{}", url),
        category,
    };
    subscription
        .insert(&clients.pool)
        .await
        .map_err(MyError::Internal)?;
    info!("{:?}", subscription);
    info!("{:#?}", channel);
    Ok(HttpResponse::Ok().json("Ok"))
}

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
struct Index<'a> {
    subscriptions: &'a Vec<DbSubscription>,
    subscription_map: HashMap<i64, &'a DbSubscription>,
    items: Vec<DbItem>,
}

#[get("/")]
#[instrument(skip(clients))]
async fn get_index(clients: web::Data<Clients>) -> Result<HttpResponse, MyError> {
    let subscriptions = DbSubscription::fetch_all(&clients.pool).await?;
    let subscription_map: HashMap<_, _> = subscriptions
        .iter()
        .filter_map(|x| Some((x.id?, x)))
        .collect();
    let items = DbItem::fetch_all(&clients.pool).await?;
    let index = Index {
        subscriptions: &subscriptions,
        subscription_map,
        items,
    };
    let body = index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("Invalid Subscription {}", .0)]
    InvalidSubscription(String, String),
    #[error("Bad Param {}", .0)]
    BadParam(String, String),
    #[error("Inernal Error")]
    Internal(#[from] Report),
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        warn!("actix response: {:?}", self);
        match self {
            MyError::Internal(x) => {
                let uuid = Uuid::new_v4();
                warn!("Internal Error ({}): {:?}", uuid, x);
                HttpResponse::with_body(
                    self.status_code(),
                    Body::from_message(format!("internal error {}", uuid)),
                )
                .into()
            }
            _ => {
                HttpResponse::with_body(self.status_code(), Body::from_message(format!("{}", self)))
                    .into()
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::BadParam(_, _) | MyError::InvalidSubscription(_, _) | MyError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

fn install_tracing() -> color_eyre::Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};
    color_eyre::install()?;

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}
