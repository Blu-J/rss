use std::time::SystemTime;

use actix_web::{cookie::Cookie, get, web, HttpResponse};
use color_eyre::eyre::eyre;
use tracing::instrument;

use crate::clients::Clients;

use super::{
    from_requests::{
        user_id::UserIdPart,
        user_preferences::{FilterItems, ShowUnreads, UserPreferences, USER_PREFERENCE},
    },
    MyError,
};

#[get("/actions/mark_all_read/{date}")]
#[instrument(skip(clients))]
pub async fn action_mark_all_read(
    clients: web::Data<Clients>,
    date_secs: web::Path<u64>,
    UserIdPart(user_id): UserIdPart,
    user_preference: UserPreferences,
) -> Result<HttpResponse, MyError> {
    let date_secs = *date_secs as i64;
    let (filter_session_id, filter_session_title) = user_preference.filter_items.as_items();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|error| MyError::Internal(eyre!("Could not get now time: {:?}", error)))?
        .as_secs() as i64;
    sqlx::query_file!(
        "queries/user_item_reads_insert.sql",
        user_id,
        date_secs,
        now,
        filter_session_id,
        filter_session_title
    )
    .execute(&clients.pool)
    .await
    .map_err(|x| eyre!("Sql Error: {:?}", x))
    .map_err(MyError::Internal)?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}
#[get("/actions/filter_all_subscriptions")]
#[instrument(skip())]
pub async fn filter_all_subscriptions(
    user_preference: UserPreferences,
) -> Result<HttpResponse, MyError> {
    let user_preference = serde_json::to_string(&UserPreferences {
        filter_items: FilterItems::All,
        ..user_preference
    })
    .ok()
    .unwrap_or_default();
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build(USER_PREFERENCE, user_preference)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}
#[get("/actions/filter_by_category/{category_id}")]
#[instrument(skip())]
pub async fn filter_by_category(
    category_id: web::Path<i64>,
    user_preference: UserPreferences,
) -> Result<HttpResponse, MyError> {
    let user_preference = serde_json::to_string(&UserPreferences {
        filter_items: FilterItems::Id(*category_id),
        ..user_preference
    })
    .ok()
    .unwrap_or_default();
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build(USER_PREFERENCE, user_preference)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}

#[get("/actions/filter_by_category/{category_title}")]
#[instrument(skip())]
pub async fn filter_by_category_title(
    category_title: web::Path<String>,
    user_preference: UserPreferences,
) -> Result<HttpResponse, MyError> {
    let user_preference = serde_json::to_string(&UserPreferences {
        filter_items: FilterItems::Title(category_title.to_owned()),
        ..user_preference
    })
    .ok()
    .unwrap_or_default();
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build(USER_PREFERENCE, user_preference)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}

#[get("/actions/collapse_sidebar")]
#[instrument(skip())]
pub async fn collapse_sidebar(user_preference: UserPreferences) -> Result<HttpResponse, MyError> {
    let user_preference = serde_json::to_string(&UserPreferences {
        sidebar_collapsed: true,
        ..user_preference
    })
    .ok()
    .unwrap_or_default();
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build(USER_PREFERENCE, user_preference)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}
#[get("/actions/expand_sidebar")]
#[instrument(skip())]
pub async fn expand_sidebar(user_preference: UserPreferences) -> Result<HttpResponse, MyError> {
    let user_preference = serde_json::to_string(&UserPreferences {
        sidebar_collapsed: false,
        ..user_preference
    })
    .ok()
    .unwrap_or_default();
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build(USER_PREFERENCE, user_preference)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}
#[get("/actions/show_unreads")]
#[instrument(skip())]
pub async fn show_unreads(user_preference: UserPreferences) -> Result<HttpResponse, MyError> {
    let user_preference = serde_json::to_string(&UserPreferences {
        show_unreads: ShowUnreads::ShowUnreads,
        ..user_preference
    })
    .ok()
    .unwrap_or_default();
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build(USER_PREFERENCE, user_preference)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}
#[get("/actions/show_everything")]
#[instrument(skip())]
pub async fn show_everything(user_preference: UserPreferences) -> Result<HttpResponse, MyError> {
    let user_preference = serde_json::to_string(&UserPreferences {
        show_unreads: ShowUnreads::ShowEverything,
        ..user_preference
    })
    .ok()
    .unwrap_or_default();
    Ok(HttpResponse::Found()
        .cookie(
            Cookie::build(USER_PREFERENCE, user_preference)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .append_header(("Location", "/"))
        .finish())
}
