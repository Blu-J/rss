use imbl::OrdMap;
use scraper::{Html, Selector};
use std::time::{Duration, SystemTime};
use tracing_subscriber::EnvFilter;

use crate::{clients::Clients, server::spawn_server};

use color_eyre::Report;
use futures::{channel::oneshot::Sender, FutureExt};

use futures::TryStreamExt;
use settings::Settings;
use sqlx::query_file;
use tracing::{error, info};

pub mod clients;
// pub mod dto;
pub mod server;
pub mod session;
pub mod settings;
pub mod utils;

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    install_tracing()?;
    info!("Hello, world!");
    let clients = Clients::new(Settings::new().unwrap()).await?;

    let server = spawn_server(clients.clone());
    server.await?;
    Ok(())
}
fn install_tracing() -> color_eyre::Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;
    color_eyre::install()?;

    let fmt_layer = fmt::layer().with_target(false);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("rss=info,warn"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}
