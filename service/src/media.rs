use config::Config;
use sqlx::SqlitePool;

use crate::{auth, error::ServiceError};

pub async fn delete_media(
    db: &SqlitePool,
    conf: &Config,
    token: &str,
    path: &str,
) -> Result<(), ServiceError> {
    if !auth::auth(db, conf, token).await {
        return Err(ServiceError::InvalidToken);
    }

    let full_path = format!("{}/{}", conf.file.root, path);

    if tokio::fs::metadata(&full_path).await.is_err() {
        return Err(ServiceError::FileNotFound(path.to_string()));
    }

    match tokio::fs::remove_file(full_path).await {
        Ok(_) => (),
        Err(_) => return Err(ServiceError::CouldNotDeleteFile(path.to_string())),
    }

    Ok(())
}
