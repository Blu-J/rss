use std::time::SystemTime;

use actix_web::{get, web, HttpResponse};
use askama::Template;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use tracing::instrument;

use crate::{clients::Clients, dto};

use super::filters;
use super::{wrap_body, MyError, UserIdPart};

#[get("/items/partial/{id}")]
#[instrument(skip(clients))]
pub async fn get_full_item_part(
    clients: web::Data<Clients>,
    id: web::Path<i64>,
    UserIdPart(user_id): UserIdPart,
) -> Result<HttpResponse, MyError> {
    let item = dto::Item::fetch(*id, &clients.pool)
        .await?
        .ok_or_else(|| MyError::Missing("Item".to_string()))?;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|error| MyError::Internal(eyre!("Could not get now time: {:?}", error)))?
        .as_secs() as i64;

    sqlx::query!(
        "INSERT OR IGNORE INTO user_item_reads (item_id, user_id, read_on) VALUES ($1, $2, $3)",
        item.id,
        user_id,
        now
    )
    .execute(&clients.pool)
    .await
    .map_err(Report::from)
    .map_err(MyError::Internal)?;
    let subscription =
        dto::UserSubscription::fetch(&user_id, item.subscription_id, &clients.pool).await?;
    let index = TemplateFullItem {
        show_expanded: true,
        subscription: &&subscription,
        item: &item,
    };
    let body = index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
#[get("/item/{id}")]
#[instrument(skip(clients))]
pub async fn get_full_item(
    clients: web::Data<Clients>,
    id: web::Path<i64>,
    UserIdPart(user_id): UserIdPart,
) -> Result<HttpResponse, MyError> {
    let item = dto::Item::fetch(*id, &clients.pool)
        .await?
        .ok_or_else(|| MyError::Missing("Item".to_string()))?;
    let subscription =
        dto::UserSubscription::fetch(&user_id, item.subscription_id, &clients.pool).await?;
    let index = wrap_body(TemplateFullItem {
        show_expanded: true,
        subscription: &&subscription,
        item: &item,
    });
    let body = index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Template, Debug, Clone)]
#[template(path = "item.html.j2")]
struct TemplateFullItem<'a> {
    item: &'a dto::Item,
    subscription: &'a dto::UserSubscription,
    show_expanded: bool,
}
