use crate::{
    clients::Clients,
    dto::User,
    session::{Session, SessionMap},
};
use actix_web::{cookie::Cookie, get, http::header::ContentType, post, web, HttpResponse};
use askama::Template;
use color_eyre::eyre::eyre;
use serde::Deserialize;
use tracing::instrument;

use uuid::Uuid;

use super::{wrap_body, MyError};

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

#[derive(Template, Debug, Clone)]
#[template(path = "login.j2")]
struct TemplateLogin;
#[get("/login")]
#[instrument]
pub async fn login_get() -> Result<HttpResponse, MyError> {
    let index = wrap_body(TemplateLogin);
    let body = index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
#[post("/login")]
#[instrument(skip())]
pub async fn login_post(
    session_maps: web::Data<SessionMap>,
    login_form: web::Form<LoginForm>,
    clients: web::Data<Clients>,
) -> Result<HttpResponse, MyError> {
    let mut session_map = session_maps.lock().await;
    let user = User::fetch(&clients.pool, &login_form.username)
        .await
        .map_err(MyError::CannotFind)?;

    if !user.passwords_match(&login_form.password) {
        return Err(MyError::CannotFind(eyre!("Cannot find")));
    }

    let ssid = Uuid::new_v4().to_string();
    session_map.insert(ssid.clone(), Session::new((*user).clone()));
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build("ssid", ssid)
                .path("/")
                // .secure(true)
                .http_only(true)
                .finish(),
        )
        .content_type(ContentType::html())
        .append_header(("Location", "/"))
        .json("Ok"))
}
