use std::time::{Duration, SystemTime};

use crate::{clients::Clients, server::template_utils::with_full_page, utils};
use actix_web::{get, web, HttpResponse};

use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::Result;
use maud::{html, Markup};
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
    let user = query_file!("queries/user_by_id.sql", user_id_part.0)
        .fetch_one(&clients.pool)
        .await
        .map_err(|x| MyError::CannotFind(x.into()))?;
    let articles = match user.tags {
        None => query_file_as!(Article, "queries/articles_all.sql", user_id_part.0)
            .fetch_all(&clients.pool)
            .await
            .map_err(|x| MyError::CannotFind(x.into()))?,
        Some(tags) => {
            let items = serde_json::to_string(&tags.split(' ').collect::<Vec<_>>()).unwrap();
            query_file_as!(
                Article,
                "queries/articles_by_tags.sql",
                user_id_part.0,
                items
            )
            .fetch_all(&clients.pool)
            .await
            .map_err(|x| MyError::CannotFind(x.into()))?
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(with_full_page(articles_page(&articles).await?).into_string()))
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
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(with_full_page(articles_page(&articles).await?).into_string()))
}

async fn articles_page(articles: &[Article]) -> Result<Markup> {
    Ok(html! {
        header {
            h1 { "Articles" }
        }
        main {
            @for article in articles {
                a.feed  href=(article.href) {
                    .figure {
                        @if let Some(src) = &article.image_src {
                            img src=(src);
                        }
                    }
                    .head {
                        (article.title)
                    }
                    .body {
                        @if let Some(body) = &article.description {
                            (body)
                        }
                    }
                    .footer{

                        (article.site_title) " / "
                        div title=(format_date(article.date)) {
                            (format_ago(article.date))
                        }
                        @if let Some(href) = &article.comments_href {
                            a href=(href) {
                                "Comments"
                            }
                        } @else {
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
