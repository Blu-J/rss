// use self::{
//     items::{get_full_item, get_full_item_part},
//     subscriptions::{new_subscription, page_all_subscriptions, page_rss_subscription_form},
// };
use crate::{clients::Clients, session::SessionMap};
use actix_web::{
    body::MessageBody, error, http::StatusCode, middleware, rt::spawn, web, App, HttpResponse,
    HttpServer,
};
use color_eyre::Report;
use futures::lock::Mutex;
use lru_time_cache::LruCache;
use std::{sync::Arc, time::Duration};
use tracing::warn;
use uuid::Uuid;

// mod actions;
// mod items;
mod login;
// mod subscriptions;
mod articles;
mod sites;
mod tags;
pub mod template_utils;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Invalid Subscription {}", .0)]
    InvalidSubscription(String, String),
    #[error("Cannot find")]
    CannotFind(Report),
    #[error("Bad Param {}", .0)]
    BadParam(String, String),
    #[error("Internal Error")]
    Internal(#[from] Report),
    #[error("Missing {}", .0)]
    Missing(String),
    #[error("User Not Logged in")]
    NotLoggedIn(Report),
}

impl MyError {
    fn internal(error: impl Into<Report>) -> Self {
        MyError::Internal(error.into())
    }
    fn bad_param(param: &str, value: &str) -> Self {
        MyError::BadParam(param.to_string(), value.to_string())
    }
}

pub fn spawn_server(clients: Clients) -> tokio::task::JoinHandle<()> {
    spawn(async move {
        let sessions: SessionMap =
            Arc::new(Mutex::new(LruCache::with_expiry_duration_and_capacity(
                Duration::from_secs(clients.settings.time_of_cookies_s),
                clients.settings.max_sessions as usize,
            )));
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(clients.clone()))
                .app_data(web::Data::new(sessions.clone()))
                .wrap(middleware::Compress::default())
                .service(login::page)
                .service(login::post)
                .service(articles::articles_default)
                .service(articles::all)
                .service(tags::tags)
                .service(tags::set_tags)
                .service(sites::duplicate_new_site)
                .service(sites::new_site)
                .service(sites::update_site)
                .service(sites::all)
                .service(sites::post_sites)
                .service(actix_files::Files::new("/static", "./static").show_files_listing())
        })
        .bind("0.0.0.0:8080")
        .expect("starting server")
        .run()
        .await
        .expect("running server");
    })
}

// fn wrap_body<A: Display>(wrapped: A) -> String {
//     templates::Home {
//         body: &format!("{}", wrapped),
//     }
//     .to_string()
// }

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        warn!("actix response: {:?}", self);
        match self {
            MyError::NotLoggedIn(_x) => {
                warn!("Redirecting");
                HttpResponse::Found()
                    .append_header(("Location", "/login"))
                    .body("Not logged in")
            }
            MyError::CannotFind(x) => {
                let uuid = Uuid::new_v4();
                warn!("Cannot Find ({}): {:?}", uuid, x);
                HttpResponse::with_body(
                    self.status_code(),
                    (format!("internal error {}", uuid)).boxed(),
                )
            }
            MyError::Internal(x) => {
                let uuid = Uuid::new_v4();
                warn!("Internal Error ({}): {:?}", uuid, x);
                HttpResponse::with_body(
                    self.status_code(),
                    (format!("internal error {}", uuid)).boxed(),
                )
            }
            _ => HttpResponse::with_body(self.status_code(), (format!("{}", self)).boxed()),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::BadParam(_, _)
            | MyError::InvalidSubscription(_, _)
            | MyError::NotLoggedIn(_)
            | MyError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::Missing(_) | MyError::CannotFind(_) => StatusCode::NOT_FOUND,
        }
    }
}

pub(crate) mod from_requests {
    pub mod user_id;
    pub mod user_preferences;
}
