use anyhow::Result;

use config::Config;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::SqlitePool;

async fn gen_otp() -> String {
    let otp: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    format!("otp-{}", otp)
}

pub async fn new_otp(pool: &SqlitePool, config: &Config, secret: &str) -> Result<String> {
    if config.auth.secret != secret {
        return Err(anyhow::anyhow!("Invalid secret"));
    }

    let otp = gen_otp().await;

    save_otp(pool, &otp).await?;

    Ok(otp)
}

pub async fn verify_otp(pool: &SqlitePool, conf: &Config, otp: &str) -> bool {
    let lifespan = format!("-{} seconds", conf.auth.otp_lifespan);
    let row = sqlx::query!(
        "SELECT * FROM otp WHERE value = $1 AND is_used = false AND created_at > datetime('now', $2)",
        otp,
        lifespan
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    if row.is_none() {
        return false;
    }

    true
}

pub async fn save_otp(pool: &SqlitePool, otp: &str) -> Result<()> {
    sqlx::query!("INSERT INTO otp (value) VALUES ($1)", otp)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn use_otp(pool: &SqlitePool, otp: &str) -> Result<()> {
    sqlx::query!("UPDATE otp SET is_used = true WHERE value = $1", otp)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_otp(pool: &SqlitePool, otp: &str) -> Result<()> {
    sqlx::query!("DELETE FROM otp WHERE value = $1", otp)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn auth(pool: &SqlitePool, conf: &Config, token: &str) -> bool {
    if conf.auth.allow_external_upload
        && token.starts_with("otp-")
        && verify_otp(pool, conf, token).await
    {
        return use_otp(pool, token).await.is_ok();
    } else if token == conf.auth.secret {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gen_otp() {
        let otp = gen_otp().await;

        assert_eq!(otp.len(), 20);

        assert!(otp.starts_with("otp-"));
    }
}
