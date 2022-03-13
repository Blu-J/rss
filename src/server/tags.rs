use std::time::{Duration, SystemTime};

use crate::{
    clients::Clients,
    server::template_utils::{with_full_page, TAG_PREFERENCE},
    utils,
};
use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::Result;
use futures::TryStreamExt;
use imbl::{ordset, OrdMap, OrdSet};
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::{query_file, query_file_as};
use time_humanize::HumanTime;
use tracing::instrument;

use super::{from_requests::user_id::UserIdPart, MyError};

#[derive(Debug, Deserialize)]
pub struct SetSites {
    action: String,
    tags: Option<Tags>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Tags {
    Single(String),
    Multiple(OrdSet<String>),
}

impl Tags {
    fn to_set(&self) -> OrdSet<String> {
        match self {
            Tags::Single(x) => ordset![x.clone()],
            Tags::Multiple(x) => x.clone(),
        }
    }
}

#[get("/tags")]
#[instrument]
pub async fn tags(
    clients: web::Data<Clients>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    let tags: OrdMap<String, u64> = query_file!("queries/site_tags_get_all.sql", user_id_part.0)
        .fetch(&clients.pool)
        .try_fold(OrdMap::default(), |mut acc, site| async move {
            let site_counter = acc.entry(site.tag).or_default();
            *site_counter += 1;
            Ok(acc)
        })
        .await
        .map_err(|x| MyError::CannotFind(x.into()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(
        with_full_page(
            html! {
                h2{"Set Tags"}
                p {
                    "Tags are how we group site scrapers together. You can either not filter or filter by selected tags."
                }
            },
            html! {
                form action="/set_tags" method="post" {
                    fieldset {
                        button type="submit" name="action" value="all" { "All Tags" }

                        h2 {"Tags"}
                        @for (tag, count) in tags {
                            label {
                                input type="radio" name="tags" value=(tag) { }
                                (format!("{}: ({})",tag, count))
                            }
                        }


                        button type="submit" name="action" value="set" {
                            "Set Tags"
                        }
                    }
                }

            },
        )
        .into_string(),
    ))
}

#[post("/set_tags")]
#[instrument]
pub async fn set_tags(
    clients: web::Data<Clients>,
    login_form: web::Form<SetSites>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    let all_tags = login_form
        .tags
        .as_ref()
        .map(|x| x.to_set())
        .unwrap_or_default()
        .into_iter()
        .fold(String::new(), |acc, next| format!("{} {}", acc, next))
        .trim()
        .to_string();
    match login_form.action.as_str() {
        "set" if !all_tags.is_empty() => {
            query_file!(
                "queries/preferences_set.sql",
                user_id_part.0,
                TAG_PREFERENCE,
                all_tags
            )
            .execute(&clients.pool)
            .await
            .map_err(|x| MyError::Internal(x.into()))?;
        }
        _ => {
            query_file!(
                "queries/preferences_delete.sql",
                user_id_part.0,
                TAG_PREFERENCE
            )
            .execute(&clients.pool)
            .await
            .map_err(|x| MyError::Internal(x.into()))?;
        }
    }

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}
