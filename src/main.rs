use std::{ops::Sub, time::Duration};

use crate::{
    clients::Clients,
    models::{DbItem, DbSubscription},
    server::spawn_server,
};
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
use futures::{select, FutureExt};
use lazy_static::lazy_static;

use settings::Settings;
use tracing::info;

pub mod clients;
pub mod models;
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
            let mut items: Vec<DbItem> = Vec::with_capacity(10);
            let mut sigup = signal(SignalKind::hangup())?;
            loop {
                select! {
                _ = timeout(Duration::from_secs(30), async {
                        items.truncate(0);
                        let start = Utc::now();
                        let mut subscriptions = DbSubscription::fetch_all(&clients.pool).await?;
                        subscriptions.reverse();
                        for subscription in subscriptions  {
                            for item in subscription.get_items().await? {
                                items.push(item);
                            }
                        }
                        let mut transaction = clients.pool.begin().await?;
                        for item in items.iter() {
                            item.insert(&mut transaction).await?;
                        }
                        transaction.commit().await?;
                        let duration = Utc::now().sub(start);
                        info!("Time to insert {} items: {}", items.len(), duration);
                        time::sleep(Duration::from_secs(10)).await;

                        Ok::<_, Report>(())
                    }).fuse() => (),
                    _ = ctrl_c().fuse() => break,
                    _ = sigup.recv().fuse() => break,
                }
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
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}
