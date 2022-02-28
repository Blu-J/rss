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
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use tracing::instrument;

use uuid::Uuid;

use super::MyError;

#[derive(Clone, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
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
        .body(with_full_page(login_form()).into_string()))
}
#[post("/login")]
#[instrument(skip())]
pub async fn post(
    session_maps: web::Data<SessionMap>,
    login_form: web::Form<LoginForm>,
    clients: web::Data<Clients>,
) -> Result<HttpResponse, MyError> {
    // // let mut session_map = session_maps.lock().await;
    // // let user = User::fetch(&clients.pool, &login_form.username)
    // //     .await
    // //     .map_err(MyError::CannotFind)?;

    // if !user.passwords_match(&login_form.password) {
    return Err(MyError::CannotFind(eyre!("Cannot find")));
    // }

    // let ssid = Uuid::new_v4().to_string();
    // session_map.insert(ssid.clone(), Session::new((*user).clone()));
    // Ok(HttpResponse::Found()
    //     .cookie(
    //         Cookie::build("ssid", ssid)
    //             .path("/")
    //             .secure(clients.settings.secure)
    //             .http_only(true)
    //             .expires(Some(
    //                 SystemTime::now()
    //                     .add(Duration::from_secs(clients.settings.time_of_cookies_s))
    //                     .into(),
    //             ))
    //             .finish(),
    //     )
    //     .append_header(("Location", "/"))
    //     .finish())
}

fn login_form() -> Markup {
    html! {
        header {
            h1 { "Login" }
        }
        main {
            form hx-post="/login" {
                fieldset {
                    input."" type="text" name="username" placeholder="Username";
                    input."" type="password" name="password" placeholder="Password";

                    button type="submit" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" {
                        "Login"
                    }
                }
            }
        }
    }
}
