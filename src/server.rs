use self::{
    items::{get_full_item, get_full_item_part},
    subscriptions::{get_all_subscriptions, new_subscription},
};
use crate::{clients::Clients, session::SessionMap};
use actix_web::{
    body::Body, error, http::StatusCode, middleware, rt::spawn, App, HttpResponse, HttpServer,
};
use color_eyre::Report;
use login::{login_get, login_post};
use std::fmt::Display;
use tracing::warn;
use uuid::Uuid;

mod actions;
mod items;
mod login;
mod subscriptions;
pub mod templates;

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

pub fn spawn_server(clients: Clients) -> tokio::task::JoinHandle<()> {
    spawn(async move {
        let sessions: SessionMap = Default::default();
        HttpServer::new(move || {
            App::new()
                .data(clients.clone())
                .data(sessions.clone())
                .wrap(middleware::Compress::default())
                .service(login_get)
                .service(login_post)
                .service(new_subscription)
                .service(actix_files::Files::new("/static", "./static").show_files_listing())
                .service(get_all_subscriptions)
                .service(get_full_item)
                .service(get_full_item_part)
                .service(actions::action_mark_all_read)
                .service(actions::filter_all_subscriptions)
                .service(actions::filter_by_category)
                .service(actions::filter_by_category_title)
                .service(actions::expand_sidebar)
                .service(actions::collapse_sidebar)
                .service(actions::show_everything)
                .service(actions::show_unreads)
        })
        .bind("0.0.0.0:8080")
        .expect("starting server")
        .run()
        .await
        .expect("running server");
    })
}

fn wrap_body<A: Display>(wrapped: A) -> String {
    templates::Home {
        body: &format!("{}", wrapped),
    }
    .to_string()
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        warn!("actix response: {:?}", self);
        match self {
            MyError::NotLoggedIn(x) => {
                warn!("Redirecting:  {:?}", x);
                HttpResponse::Found()
                    .append_header(("Location", "/login"))
                    .body("Not logged in")
            }
            MyError::CannotFind(x) => {
                let uuid = Uuid::new_v4();
                warn!("Cannot Find ({}): {:?}", uuid, x);
                HttpResponse::with_body(
                    self.status_code(),
                    Body::from_message(format!("internal error {}", uuid)),
                )
            }
            MyError::Internal(x) => {
                let uuid = Uuid::new_v4();
                warn!("Internal Error ({}): {:?}", uuid, x);
                HttpResponse::with_body(
                    self.status_code(),
                    Body::from_message(format!("internal error {}", uuid)),
                )
            }
            _ => {
                HttpResponse::with_body(self.status_code(), Body::from_message(format!("{}", self)))
            }
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
