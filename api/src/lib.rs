use anyhow::Result;
use config::{load_config, Config};
use service::{prepare, prepare_db, SqlitePool, SqlitePoolOptions};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;

mod error;
mod route;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
}

#[tokio::main]
async fn start() -> Result<()> {
    let conf = load_config().await?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    prepare(&conf).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&conf.database.url)
        .await
        .expect("can't connect to database");

    prepare_db(&pool).await?;

    let app_state = AppState {
        db: pool,
        config: conf.clone(),
    };
    let app = route::build(app_state);

    let listener = TcpListener::bind(format!("{}:{}", conf.server.host, conf.server.port)).await?;

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
