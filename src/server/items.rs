use actix_web::{HttpResponse, get, web};
use tracing::instrument;
use askama::Template;

use crate::{clients::Clients, dto};

use super::{MyError, User, wrap_body};
use super::filters;



#[get("/items/partial/{id}")]
#[instrument(skip(clients))]
pub async fn get_full_item_part(
    clients: web::Data<Clients>,
    id: web::Path<i64>,
    User(user_id): User,
) -> Result<HttpResponse, MyError> {
    let item = dto::Item::fetch(*id, &clients.pool)
        .await?
        .ok_or_else(|| MyError::Missing("Item".to_string()))?;
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
    User(user_id): User,
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