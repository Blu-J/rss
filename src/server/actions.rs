use std::time::{ SystemTime};

use actix_web::{HttpResponse,  post, web};
use tracing::instrument;
use color_eyre::eyre::eyre;

use crate::{clients::Clients};

use super::{MyError, UserIdPart};



#[post("/actions/mark_all_read/{date}")]
#[instrument(skip(clients))]
pub async fn action_mark_all_read(
    clients: web::Data<Clients>,
    date_secs: web::Path<u64>,
    UserIdPart(user_id): UserIdPart,
) -> Result<HttpResponse, MyError> {
    let date_secs = *date_secs as i64;
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map_err(|error|MyError::Internal(eyre!("Could not get now time: {:?}", error)))?.as_secs() as i64;
    sqlx::query!("INSERT INTO user_item_reads (item_id, user_id, read_on) SELECT id, $1, $3 FROM items WHERE pub_date <= $2", user_id, date_secs, now).execute(&clients.pool).await.map_err(|x| eyre!("Sql Error: {:?}", x)).map_err(MyError::Internal)?;

    Ok(HttpResponse::Found()
    .append_header(("Location", "/")).finish())
}