use std::{
    ops::Add,
    time::{Duration, SystemTime},
};

use crate::{
    clients::Clients,
    server::template_utils::with_full_page,
    session::{Session, SessionMap},
};
use actix_web::{get, web, HttpResponse};

use color_eyre::Result;
use maud::{html, Markup};
use sqlx::query_file;
use tracing::instrument;

use super::{from_requests::user_id::UserIdPart, MyError};

#[get("/articles")]
#[instrument]
pub async fn all(
    clients: web::Data<Clients>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(with_full_page(articles_page(&clients, user_id_part).await?).into_string()))
}

async fn articles_page(clients: &Clients, user_id_part: UserIdPart) -> Result<Markup> {
    let articles = query_file!("queries/articles_all.sql", user_id_part.0)
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
