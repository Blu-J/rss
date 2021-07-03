

use crate::{dto};
use actix_web::{cookie::Cookie, get,post,web,  HttpResponse, 
};
use askama::Template;
use serde::Deserialize;
use tracing::{ instrument};

use uuid::Uuid;

use super::{MyError, SessionMap, wrap_body};

#[derive(Debug, Clone, Deserialize)]
pub struct FormLogin {
    username: String,
    password: String,
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
pub async fn login_post(session_maps: web::Data<SessionMap>) -> Result<HttpResponse, MyError> {
    let mut session_map = session_maps.lock().await;
    let ssid = Uuid::new_v4().to_string();
    session_map.insert(ssid.clone(), dto::UserId(1));
    // info!("Starting a new subscription {:?}", path);
    // let SubscriptionForm {
    //     category,
    //     title,
    //     url,
    // } = path.into_inner();

    // let content = reqwest::get(url.clone())
    //     .await
    //     .map_err(|e| MyError::BadParam("url".into(), format!("{:?}", e)))?
    //     .bytes()
    //     .await
    //     .map_err(|e| MyError::BadParam("url".into(), format!("{:?}", e)))?;
    // let channel = Channel::read_from(&content[..])
    //     .map_err(|x| MyError::InvalidSubscription(url.clone(), x.to_string()))?;
    // let rss_feed = format!("{}", url);
    // let subscription = dto::Subscription::insert(&&rss_feed, &clients.pool).await?;
    // let subscription = dto::UserSubscription::insert(&&rss_feed, &clients.pool).await?;
    // let subscription = dto::UserSubscription {
    //     id: None,
    //     title,
    //     rss_feed: format!("{}", url),
    //     category,
    //     unreads: None,
    // };
    // subscription
    //     .insert(&clients.pool)
    //     .await
    //     .map_err(MyError::Internal)?;
    // info!("{:?}", subscription);
    // info!("{:#?}", channel);
    Ok(HttpResponse::TemporaryRedirect()
        .cookie(
            Cookie::build("ssid", ssid)
                .path("/")
                .secure(true)
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .json("Ok"))
}
