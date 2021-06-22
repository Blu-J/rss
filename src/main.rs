use std::{ops::Sub, time::Duration};

use crate::{clients::Clients, server::spawn_server};
use actix_web::rt::{
    signal::{
        ctrl_c,
        unix::{signal, SignalKind},
    },
    spawn,
    time::{self, timeout},
};

use chrono::Utc;
use color_eyre::Report;
use futures::{select, stream, FutureExt, StreamExt};
use lazy_static::lazy_static;

use settings::Settings;
use tracing::{error, info, warn};

pub mod clients;
pub mod dto;
pub mod server;
pub mod settings;

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().unwrap();
}

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    install_tracing()?;
    info!("Hello, world!");
    let clients = Clients::new("./data.db").await?;

    let server = spawn_server(clients.clone());
    let task2 = spawn(async move {
        async {
            let mut sigup = signal(SignalKind::hangup())?;
            loop {
                match select! {
                    x = timeout(Duration::from_secs(60), async {
                        let start = Utc::now();
                        let items_to_insert: Vec<_> = stream::iter(dto::Subscription::fetch_all(&clients.pool).await?.into_iter().map(|subscription| async move{
                            Ok::<_, Report>(subscription.get_items().await?)
                        })).buffer_unordered(10).filter_map(|x| async {match x {
                            Ok(x) => Some(x),
                            Err(e) => {
                                warn!("Ran into issues getting rss: {:?}", e);
                                None 
                            }
                        }}).concat().await;
                        
                        let mut transaction = clients.pool.begin().await?;
                        for item in items_to_insert.iter() {
                            item.insert(&mut transaction).await?;
                        }
                        transaction.commit().await?;
                        let duration = Utc::now().sub(start);
                        info!("Time to insert {} items: {}", items_to_insert.len(), duration);

                        time::sleep(Duration::from_secs(10)).await;
                        Ok::<_, Report>(())
                    }).fuse() => x,
                    _ = ctrl_c().fuse() => break,
                    _ = sigup.recv().fuse() => break,
                } { 
                    Err(e) => {error!("Item Insert Daemon timeout: {:?}", e);},
                    Ok(Err(e)) => {error!("Item Insert Daemon error: {:?}", e);},
                    _ => ()
                };
            }

            Ok::<_, Report>(())
        }
        .await
        .expect("Running in task 2");
    });
    server.await?;
    task2.await?;
    Ok(())
}
fn install_tracing() -> color_eyre::Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};
    color_eyre::install()?;

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("rss::info,warn"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}
