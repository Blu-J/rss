use std::{collections::HashMap, fmt::Display, sync::Arc};
use crate::{clients::Clients, dto};
// use actix_session::CookieSession;
use actix_web::{
    body::Body,  dev::Payload, error,  http::StatusCode, middleware, 
    rt::spawn, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer,
};
use askama::Template;
use color_eyre::{eyre::eyre, Report};
use futures::{future::LocalBoxFuture, FutureExt};
use tokio::sync::Mutex;
use tracing::{  warn};
use uuid::Uuid;
use login::{login_get, login_post};
use self::{items::{get_full_item, get_full_item_part}, subscriptions::{get_all_subscriptions, new_subscription}};

mod login;
mod items;
mod subscriptions;
mod filters;

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
                .service(new_subscription)
                .service(actix_files::Files::new("/static", "./static").show_files_listing())
                .service(get_all_subscriptions)
                .service(get_full_item)
                .service(login_get)
                .service(login_post)
                .service(get_full_item_part)
        })
        .bind("0.0.0.0:8080")
        .expect("starting server")
        .run()
        .await
        .expect("running server");
    })
}

#[derive(Template, Debug, Clone)]
#[template(path = "full_body.html.j2")]
struct FullBody<A: Template + Display> {
    wrapped: A,
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
            MyError::BadParam(_, _)
            | MyError::InvalidSubscription(_, _)
            | MyError::NotLoggedIn(_)
            | MyError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::Missing(_) => StatusCode::NOT_FOUND,
        }
    }
}


type SessionMap = Arc<Mutex<HashMap<String, dto::UserId>>>;

#[derive(Debug, Clone)]
pub struct User(pub dto::UserId);

impl<'a> FromRequest for User {
    type Error = MyError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let ssid = dbg!(req.cookie("ssid"));
        let value = web::Data::<SessionMap>::from_request(req, payload)
            .then(|session_map| async move {
                let session = ssid.ok_or_else(|| eyre!("Need to have a SSID cookie"))?;
                let session = session.value();
                let sessions_data = session_map.map_err(|_| eyre!("Could not extract sessions"))?;
                let sessions = sessions_data.lock().await;
                let user_id = sessions
                    .get(&session.to_string())
                    .ok_or_else(|| eyre!("No cookie in sessions"))?;

                Ok(Self(user_id.clone()))
            })
            .map(|x| x.map_err(MyError::NotLoggedIn));
        Box::pin(value)
    }
}
