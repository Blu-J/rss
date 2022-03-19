use std::{
    char::MAX,
    time::{Duration, SystemTime},
};

use crate::{
    clients::Clients,
    server::template_utils::{with_full_page, TAG_PREFERENCE},
    utils,
};
use actix_web::{get, web, HttpResponse};

use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::Result;
use maud::{html, Markup, PreEscaped};
use sqlx::{query_file, query_file_as};
use time_humanize::HumanTime;
use tracing::instrument;

use super::{from_requests::user_id::UserIdPart, MyError};

#[derive(Debug)]
struct Article {
    href: String,
    title: String,
    description: Option<String>,
    date: u32,
    image_src: Option<String>,
    comments_href: Option<String>,
    site_title: String,
}

#[get("/")]
#[instrument]
pub async fn articles_default(
    clients: web::Data<Clients>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    let user_tags = query_file!(
        "queries/preferences_get.sql",
        user_id_part.0,
        TAG_PREFERENCE
    )
    .fetch_optional(&clients.pool)
    .await
    .map_err(|x| MyError::CannotFind(x.into()))?
    .map(|x| x.value);
    let mut title = "All Articles".to_string();
    let articles = match user_tags {
        None => query_file_as!(Article, "queries/articles_all.sql", user_id_part.0)
            .fetch_all(&clients.pool)
            .await
            .map_err(|x| MyError::CannotFind(x.into()))?,
        Some(tag) => {
            title = format!("Articles with tag: {}", tag);
            dbg!(&tag);
            query_file_as!(Article, "queries/articles_by_tags.sql", user_id_part.0, tag)
                .fetch_all(&clients.pool)
                .await
                .map_err(|x| MyError::CannotFind(x.into()))?
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(with_full_page(html! {h2{(title)}}, articles_page(&articles).await?).into_string()))
}

#[instrument]
#[get("/all")]
pub async fn all(
    clients: web::Data<Clients>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    let articles = query_file_as!(Article, "queries/articles_all.sql", user_id_part.0)
        .fetch_all(&clients.pool)
        .await
        .map_err(|x| MyError::CannotFind(x.into()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(
        with_full_page(html! {h2{"All Articles"}}, articles_page(&articles).await?).into_string(),
    ))
}

async fn articles_page(articles: &[Article]) -> Result<Markup> {
    let break_slash: Markup = PreEscaped("&nbsp;/&nbsp;".to_string());
    Ok(html! {
        @for article in articles {
            .article.shadowed {
                .figure {
                    @if let Some(src) = &article.image_src {
                        img src=(src) loading="lazy" alt=(article.title);
                    }
                }
                .non-figure {
                    h3.head {
                        a.feed  href=(article.href) {
                            (article.title)
                        }
                    }
                    .body {
                        @if let Some(body) = &article.description {
                            (body)
                        }
                    }
                    .footer{

                        (article.site_title)
                        (break_slash.clone())
                        div title=(format_date(article.date)) {
                            (format_ago(article.date))
                        }
                        @if let Some(href) = &article.comments_href {
                            (break_slash.clone())
                            a href=(href) {
                                "Comments"
                            }
                        } @else {
                            (break_slash.clone())
                            "No Comments"
                        }
                    }
                }
            }
        }
    })
}

fn format_ago(date_time: u32) -> String {
    let now = utils::unix_time();

    format!("{}", HumanTime::from(date_time as i64 - now as i64))
}

fn format_date(date_time: u32) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(date_time as i64, 0);

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    // Format the datetime how you want
    let formated_date = datetime.format("%Y-%m-%d %H:%M:%S");
    format!("{}", formated_date)
}
