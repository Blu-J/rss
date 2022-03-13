use std::{
    ops::Add,
    time::{Duration, SystemTime},
};

use crate::{
    clients::Clients,
    server::template_utils::with_full_page,
    session::{Session, SessionMap},
};
use actix_web::{cookie::Cookie, get, post, web, HttpResponse};
use color_eyre::eyre::eyre;
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::query_file;
use tracing::instrument;

use uuid::Uuid;

use super::MyError;

#[derive(Clone, Deserialize)]
pub struct LoginForm {
    username: String,
}

impl std::fmt::Debug for LoginForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormLogin")
            .field("username", &self.username)
            .field("password", &"redacted")
            .finish()
    }
}

#[get("/login")]
#[instrument]
pub async fn page() -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(with_full_page(html! {}, login_form()).into_string()))
}
#[post("/login")]
#[instrument(skip())]
pub async fn post(
    session_maps: web::Data<SessionMap>,
    login_form: web::Form<LoginForm>,
    clients: web::Data<Clients>,
) -> Result<HttpResponse, MyError> {
    let user = query_file!("queries/user_find.sql", login_form.username)
        .fetch_optional(&clients.pool)
        .await
        .map_err(|x| MyError::CannotFind(x.into()))?;

    let user = match user {
        Some(user) => user,
        None => return Err(MyError::CannotFind(eyre!("Cannot find"))),
    };

    let ssid = Uuid::new_v4().to_string();
    session_maps
        .lock()
        .await
        .insert(ssid.clone(), Session::new(user.id));
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build("ssid", ssid)
                .path("/")
                .secure(clients.settings.secure)
                .http_only(true)
                .expires(Some(
                    SystemTime::now()
                        .add(Duration::from_secs(clients.settings.time_of_cookies_s))
                        .into(),
                ))
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}

fn login_form() -> Markup {
    html! {
        form action="/login"  method="post" {
            fieldset {
                input."" type="text" name="username" placeholder="Username";

                button type="submit" {
                    "Login"
                }
            }
        }
    }
}
