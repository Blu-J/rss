use std::collections::HashMap;

use crate::{clients::Clients, dto, server::MyError};
use actix_web::{get, post, web, HttpResponse};
use rss::Channel;
use serde::Deserialize;
use tracing::{info, instrument};

use super::{
    from_requests::{user_id::UserIdPart, user_preferences::UserPreferences},
    templates, wrap_body,
};

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

    let user_id = dto::UserId(1);

    let content = reqwest::get(url.clone())
        .await
        .map_err(|e| MyError::BadParam("url".into(), format!("{:?}", e)))?
        .bytes()
        .await
        .map_err(|e| MyError::BadParam("url".into(), format!("{:?}", e)))?;
    let _channel = Channel::read_from(&content[..])
        .map_err(|x| MyError::InvalidSubscription(url.clone(), x.to_string()))?;
    let rss_feed = url.to_string();
    let subscription = dto::Subscription::insert(&rss_feed, &clients.pool).await?;
    let _user_subscription =
        dto::UserSubscription::insert(&category, &title, &subscription, &user_id, &clients.pool)
            .await?;

    Ok(HttpResponse::Ok().json("Ok"))
}

#[get("/")]
#[instrument(skip(clients))]
pub async fn get_all_subscriptions(
    clients: web::Data<Clients>,
    UserIdPart(user_id): UserIdPart,
    user_preference: UserPreferences,
) -> Result<HttpResponse, MyError> {
    let subscriptions = dto::UserSubscription::fetch_all(&user_id, &clients.pool).await?;
    let subscription_map: HashMap<_, _> = subscriptions.iter().map(|x| (x.id, x)).collect();
    let items = dto::Item::fetch_all_not_read(
        &user_id,
        &user_preference.filter_items,
        &user_preference.show_unreads,
        &clients.pool,
    )
    .await?;
    let subscriptions_read: HashMap<i64, String> = subscriptions
        .iter()
        .map(|subscription| {
            (
                subscription.id,
                items
                    .iter()
                    .filter(|x| x.subscription_id == subscription.id)
                    .count()
                    .to_string(),
            )
        })
        .collect();
    let index = wrap_body(templates::AllSubscriptions {
        latest_read: items
            .iter()
            .map(|x| x.pub_date as i64)
            .max()
            .unwrap_or_default(),
        subscriptions: subscriptions.iter().collect(),
        subscription_map,
        subscriptions_read,
        items: items.iter().collect(),
        show_unreads: user_preference.show_unreads,
        sidebar_collapsed: user_preference.sidebar_collapsed,
    });
    Ok(HttpResponse::Ok().content_type("text/html").body(index))
}

#[derive(Debug, Deserialize, Clone)]
pub struct SubscriptionForm {
    category: String,
    title: String,
    url: String,
}
