use crate::{
    clients::{scraper::get_collections, Clients},
    server::template_utils::with_full_page,
};
use actix_web::{get, post, web, HttpResponse};
use maud::{html, Markup};
use scraper::Selector;
use serde::Deserialize;
use sqlx::{query, query_file};
use tracing::instrument;

use super::{from_requests::user_id::UserIdPart, MyError};

#[derive(Debug, Deserialize)]
pub struct ScrapingSite {
    id: Option<String>,
    every_seconds: String,
    url: String,
    articles_sel: String,
    title_sel: String,
    link_sel: String,
    site_title: String,
    tags: String,
    image_sel: Option<String>,
    description_sel: Option<String>,
    comments_sel: Option<String>,
}
impl Default for ScrapingSite {
    fn default() -> Self {
        Self {
            id: None,
            every_seconds: 1.to_string(),
            url: "htttps://url.to.fetch".to_string(),
            articles_sel: "#selector.for .articles".to_string(),
            title_sel: "#selector.for .titles".to_string(),
            link_sel: "#selector.for .link".to_string(),
            site_title: "Site Title".to_string(),
            tags: "unknown".to_string(),
            image_sel: None,
            description_sel: None,
            comments_sel: None,
        }
    }
}

#[get("/sites")]
#[instrument]
pub async fn all(
    clients: web::Data<Clients>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    let sites: Vec<_> = query_file!("queries/sites_get_all.sql", user_id_part.0)
        .fetch_all(&clients.pool)
        .await
        .map_err(|x| MyError::CannotFind(x.into()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(
        with_full_page(
            html! {h2{"Sites"}},
            html! {
                ul {
                    li {
                        a href="/sites/new" { "New Site" }
                    }
                    @for site in sites {
                        li {
                            a href=(format!("/sites/{}", site.id)) { (site.site_title)}
                        }
                    }
                }

            },
        )
        .into_string(),
    ))
}

#[get("/sites/new")]
#[instrument]
pub async fn new_site(
    clients: web::Data<Clients>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        with_full_page(
            html! {h2{"New Scraper"}},
            template_create_update(&ScrapingSite::default(), None),
        )
        .into_string(),
    ))
}

#[get("/sites/{id}")]
#[instrument]
pub async fn update_site(
    clients: web::Data<Clients>,
    path: web::Path<i64>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    tracing::info!("Should be an update site");
    let site_id = path.into_inner();
    let site = query_file!("queries/sites_by_id.sql", site_id, user_id_part.0)
        .fetch_one(&clients.pool)
        .await
        .map_err(|x| MyError::CannotFind(x.into()))?;
    let site = ScrapingSite {
        id: Some(format!("{}", site.id)),
        every_seconds: site.every_seconds.to_string(),
        url: site.url,
        articles_sel: site.articles_sel,
        title_sel: site.title_sel,
        link_sel: site.link_sel,
        site_title: site.site_title,
        tags: site.tags.clone().unwrap_or_default(),
        image_sel: site.image_sel,
        description_sel: site.description_sel,
        comments_sel: site.comments_sel,
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(
        with_full_page(
            html! {h2{(format!("Scraper: {}", site.site_title))}},
            template_create_update(&site, None),
        )
        .into_string(),
    ))
}

fn template_create_update(site: &ScrapingSite, message: Option<Markup>) -> Markup {
    html! {
        form action="/sites" method="post" {

            @if let Some (message) = message{
                (message)
            }

            @if let Some (id) = site.id.as_ref() {
                input type="hidden" name="id" value=(id);
            }

            label {
                "Site Title: "
                input type="text" name="site_title" value=(site.site_title) required;
            }
            label {
                "Run Every Seconds: "
                input type="number" name="every_seconds" min="1" step="1" value=(site.every_seconds) required;
            }
            label {
                "Url: "
                input type="text" name="url" value=(site.url) required;
            }
            label {
                "Articles Selector: "
                input type="text" name="articles_sel" value=(site.articles_sel) required;
            }
            label {
                "Title Selector: "
                input type="text" name="title_sel" value=(site.title_sel) required;
            }
            label {
                "Link Selector: "
                input type="text" name="link_sel" value=(site.link_sel) required;
            }
            label {
                "Image Selector: "
                input type="text" name="image_sel" value=(site.image_sel.clone().unwrap_or_default());
            }
            label {
                "Description Selector: "
                input type="text" name="description_sel" value=(site.description_sel.clone().unwrap_or_default());
            }
            label {
                "Comments Selector: "
                input type="text" name="comments_sel" value=(site.comments_sel.clone().unwrap_or_default());
            }

            label {
                "Tags: "
                input type="text" name="tags" value=(site.tags) required pattern="([ ]?[a-zA-Z0-9])+";
            }

            a href="/sites" { "Cancel" }
            button type="submit" name="action" value="check" {"Check" }
            @if site.id.is_none() {
                button type="submit" name="action" value="create" {"Create" }
            } @else {
                button type="submit" name="action" value="update" {"Update" }
                br;
                button type="submit" name="action" value="delete" {"Delete" }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FormPostSites {
    action: String,
    #[serde(flatten)]
    scraping_site: ScrapingSite,
}

#[post("/sites")]
#[instrument]
pub async fn post_sites(
    clients: web::Data<Clients>,
    form: web::Form<FormPostSites>,
    user_id_part: UserIdPart,
) -> Result<HttpResponse, MyError> {
    match form.action.as_str() {
        "delete" => {
            query_file!(
                "queries/sites_delete.sql",
                form.scraping_site.id,
                user_id_part.0
            )
            .execute(&clients.pool)
            .await
            .map_err(|x| MyError::Internal(x.into()))?;
            query!(
                "DELETE FROM site_tags WHERE site_tags.site_id = $1",
                form.scraping_site.id
            )
            .execute(&clients.pool)
            .await
            .map_err(|x| MyError::Internal(x.into()))?;
            if let Some(id) = form
                .scraping_site
                .id
                .as_ref()
                .and_then(|x| x.parse::<i64>().ok())
            {
                clients.scraper.remove_scraper(id).await;
            }
        }
        "check" => {
            let collections = get_collections(
                &form
                    .scraping_site
                    .url
                    .parse()
                    .map_err(|_| MyError::bad_param("url", &form.scraping_site.url))?,
                &Selector::parse(&form.scraping_site.articles_sel).map_err(|_| {
                    MyError::bad_param("articles_sel", &form.scraping_site.articles_sel)
                })?,
                &Selector::parse(&form.scraping_site.link_sel)
                    .map_err(|_| MyError::bad_param("link_sel", &form.scraping_site.link_sel))?,
                &Selector::parse(&form.scraping_site.title_sel)
                    .map_err(|_| MyError::bad_param("title_sel", &form.scraping_site.title_sel))?,
                form.scraping_site
                    .description_sel
                    .as_ref()
                    .filter(|x| !x.is_empty())
                    .map(|x| {
                        Selector::parse(x).map_err(|_| MyError::bad_param("description_sel", x))
                    })
                    .transpose()?
                    .as_ref(),
                form.scraping_site
                    .image_sel
                    .as_ref()
                    .filter(|x| !x.is_empty())
                    .map(|x| Selector::parse(x).map_err(|_| MyError::bad_param("image_sel", x)))
                    .transpose()?
                    .as_ref(),
                form.scraping_site
                    .comments_sel
                    .as_ref()
                    .filter(|x| !x.is_empty())
                    .map(|x| Selector::parse(x).map_err(|_| MyError::bad_param("comments_sel", x)))
                    .transpose()?
                    .as_ref(),
            )
            .await;

            return Ok(HttpResponse::Ok().content_type("text/html").body(
                with_full_page(
                    html! {h2{(format!("Checking Scraper: {}", form.scraping_site.site_title))}},
                    template_create_update(
                        &form.scraping_site,
                        Some(html! {
                            table {
                                thead {
                                    tr{
                                        th{
                                            "Href"
                                        }

                                        th{
                                            "Title"
                                        }
                                        th{
                                            "Description"
                                        }
                                        th{
                                            "Image"
                                        }
                                        th{
                                            "Comments"
                                        }
                                    }
                                }

                            @for article in collections.into_iter().flatten() {
                                tr {
                                    td {
                                        a href=(article.href) {
                                            (article.href)
                                        }
                                    }
                                    td {
                                        (article.title)
                                    }
                                    td {
                                        (article.description.unwrap_or_default())
                                    }
                                    td {
                                        (article.image.unwrap_or_default())
                                    }
                                    td {
                                        (article.comments.unwrap_or_default())
                                    }
                                }

                            }
                        }
                        }),
                    ),
                )
                .into_string(),
            ));
        }
        "create" | "update" => {
            let form = &form.scraping_site;
            let mut transaction = clients.pool.begin().await.map_err(MyError::internal)?;

            let row = query_file!(
                "queries/sites_set.sql",
                form.id,
                user_id_part.0,
                form.every_seconds,
                form.url,
                form.articles_sel,
                form.title_sel,
                form.link_sel,
                form.site_title,
                form.image_sel,
                form.description_sel,
                form.comments_sel
            )
            .fetch_one(&mut transaction)
            .await
            .map_err(|x| MyError::Internal(x.into()))?;

            query!("DELETE FROM site_tags WHERE site_tags.site_id = $1", row.id)
                .execute(&mut transaction)
                .await
                .map_err(|x| MyError::Internal(x.into()))?;
            for tag in form.tags.split(' ') {
                query_file!("queries/site_tags_upsert.sql", row.id, tag)
                    .fetch_one(&mut transaction)
                    .await
                    .map_err(|x| MyError::Internal(x.into()))?;
            }

            transaction.commit().await.map_err(MyError::internal)?;

            clients.scraper.remove_scraper(row.id).await;

            clients.scraper.scrape_site(row.id, user_id_part.0).await?;
        }
        x => {
            return Err(MyError::BadParam("action".to_string(), x.to_string()));
        }
    }

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(with_full_page(html! {h2{"Nothing"}}, html! { "NOTHING YET "}).into_string()))
}
