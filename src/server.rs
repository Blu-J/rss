use std::{collections::HashMap, fmt::Display};

use crate::{
    clients::Clients,
    models::{DbItem, DbSubscription},
};
use actix_web::{
    body::Body, error, get, http::StatusCode, middleware, post, rt::spawn, web, App, HttpResponse,
    HttpServer,
};
use askama::Template;

use color_eyre::Report;

use rss::Channel;
use serde::Deserialize;
use tracing::{info, instrument, warn};

use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct SubscriptionForm {
    category: String,
    title: String,
    url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Invalid Subscription {}", .0)]
    InvalidSubscription(String, String),
    #[error("Bad Param {}", .0)]
    BadParam(String, String),
    #[error("Internal Error")]
    Internal(#[from] Report),
    #[error("Missing {}", .0)]
    Missing(String),
}

pub fn spawn_server(clients: Clients) -> tokio::task::JoinHandle<()> {
    spawn(async move {
        HttpServer::new(move || {
            App::new()
                .data(clients.clone())
                .wrap(middleware::Compress::default())
                .service(new_subscription)
                .service(actix_files::Files::new("/static", "./static").show_files_listing())
                .service(get_all_subscriptions)
                .service(get_full_item)
                .service(get_full_item_part)
        })
        .bind("0.0.0.0:8080")
        .expect("starting server")
        .run()
        .await
        .expect("running server");
    })
}

#[post("/subscriptions")]
#[instrument(skip(clients))]
pub async fn new_subscription(
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
        unreads: None,
    };
    subscription
        .insert(&clients.pool)
        .await
        .map_err(MyError::Internal)?;
    info!("{:?}", subscription);
    info!("{:#?}", channel);
    Ok(HttpResponse::Ok().json("Ok"))
}

#[get("/")]
#[instrument(skip(clients))]
pub async fn get_all_subscriptions(clients: web::Data<Clients>) -> Result<HttpResponse, MyError> {
    let subscriptions = DbSubscription::fetch_all(&clients.pool).await?;
    let subscription_map: HashMap<_, _> = subscriptions
        .iter()
        .filter_map(|x| Some((x.id?, x)))
        .collect();
    let items = DbItem::fetch_all(&clients.pool).await?;
    let index = wrap_body(AllSubscriptions {
        subscriptions: &subscriptions,
        subscription_map,
        items,
    });
    let body = index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/partial/item/{id}")]
#[instrument(skip(clients))]
pub async fn get_full_item_part(
    clients: web::Data<Clients>,
    id: web::Path<i64>,
) -> Result<HttpResponse, MyError> {
    let item = DbItem::fetch(*id, &clients.pool)
        .await?
        .ok_or_else(|| MyError::Missing("Item".to_string()))?;
    let subscription = DbSubscription::fetch(item.subscription_id, &clients.pool)
        .await?
        .ok_or_else(|| MyError::Missing("Subscription".to_string()))?;
    let index = TemplateFullItem { subscription, item };
    let body = index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
#[get("/item/{id}")]
#[instrument(skip(clients))]
pub async fn get_full_item(
    clients: web::Data<Clients>,
    id: web::Path<i64>,
) -> Result<HttpResponse, MyError> {
    let item = DbItem::fetch(*id, &clients.pool)
        .await?
        .ok_or_else(|| MyError::Missing("Item".to_string()))?;
    let subscription = DbSubscription::fetch(item.subscription_id, &clients.pool)
        .await?
        .ok_or_else(|| MyError::Missing("Subscription".to_string()))?;
    let index = wrap_body(TemplateFullItem { subscription, item });
    let body = index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Template, Debug, Clone)]
#[template(path = "full_body.html")]
struct FullBody<A: Template + Display> {
    wrapped: A,
}

#[derive(Template, Debug, Clone)]
#[template(path = "all_subscriptions.html")]
struct AllSubscriptions<'a> {
    subscriptions: &'a Vec<DbSubscription>,
    subscription_map: HashMap<i64, &'a DbSubscription>,
    items: Vec<DbItem>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "item.html")]
struct TemplateFullItem {
    item: DbItem,
    subscription: DbSubscription,
}
fn wrap_body<A: Template + Display>(wrapped: A) -> FullBody<A> {
    FullBody { wrapped }
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
            MyError::Missing(_) => StatusCode::NOT_FOUND,
        }
    }
}

mod filters {
    use ammonia::Builder;

    pub fn ammonia(s: &str) -> ::askama::Result<String> {
        Ok(Builder::default()
            .set_tag_attribute_value("img", "loading", "lazy")
            .clean(s)
            .to_string())
    }
}
