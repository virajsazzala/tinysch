use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::Path;
use tokio::fs;

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_file = "data/tinysch.db";

    if let Some(parent) = Path::new(db_file).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).await.expect("Failed to create db directory");
        }
    }

    if !Path::new(db_file).exists() {
        fs::File::create(db_file).await.expect("Failed to create db file");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", db_file))
        .await?;

    Ok(pool)
}