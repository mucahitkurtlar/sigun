pub mod auth;
pub mod dto;
pub mod error;
pub mod media;

use anyhow::Result;
use config::Config;
pub use mime;
use sqlx::migrate::MigrateDatabase;
pub use sqlx::postgres::{PgPool, PgPoolOptions};
pub use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Sqlite;

pub async fn prepare(config: &Config) -> Result<()> {
    tracing::info!("Preparing directories...");
    let sub_dirs: Vec<&str> = config.file.subdirs.iter().map(|s| s.as_str()).collect();
    init_dirs(&config.file.root, &sub_dirs).await?;

    if !Sqlite::database_exists(config.database.url.as_str()).await? {
        tracing::info!("Database does not exist. Creating...");
        Sqlite::create_database(config.database.url.as_str()).await?;
    }

    Ok(())
}

pub async fn prepare_db(db: &SqlitePool) -> Result<()> {
    tracing::info!("Migrating database...");
    let migrations = sqlx::migrate!("../migrations");
    migrations.run(db).await?;

    Ok(())
}

pub async fn init_dirs(root: &str, sub_dirs: &[&str]) -> Result<()> {
    if !std::path::Path::new(root).exists() {
        std::fs::create_dir_all(root)?;
    }
    for dir in sub_dirs {
        let path = format!("{}/{}", root, dir);
        if !std::path::Path::new(&path).exists() {
            std::fs::create_dir_all(&path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_dirs() {
        let root = "test_root";
        let sub_dirs = &["test_sub1", "test_sub2"];

        init_dirs(root, sub_dirs).await.unwrap();

        assert!(std::path::Path::new(root).exists());

        for dir in sub_dirs {
            let path = format!("{}/{}", root, dir);
            assert!(std::path::Path::new(&path).exists());
        }

        std::fs::remove_dir_all(root).unwrap();
    }
}
