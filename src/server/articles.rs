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
use color_eyre::Result;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use sqlx::query_file;
use tracing::instrument;

use uuid::Uuid;

use super::MyError;

#[get("/articles")]
#[instrument]
pub async fn all(clients: web::Data<Clients>) -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(with_full_page(articles_page(&clients).await?).into_string()))
}

async fn articles_page(clients: &Clients) -> Result<Markup> {
    let articles = query_file!("queries/articles.sql")
        .fetch_all(&clients.pool)
        .await?;
    Ok(html! {
        header {
            h1 { "Articles" }
        }
        main {
            dl {
                @for article in articles {
                    dt {
                        a href=(article.href){
                            (article.title)
                        }
                    }
                    dd {
                        @if let Some(body) = article.description {
                            (body)
                        }
                    }
                }
            }
        }
    })
}
