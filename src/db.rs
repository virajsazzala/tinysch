use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::Path;
use tokio::fs;


pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = "sqlite://data/tinysch.db";
    let db_file = "data/tinysch.db";

    if !Path::new(db_file).exists() {
        if let Some(parent) = Path::new(db_file).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).await.expect("Failed to create db directory");
            }
        }
        fs::File::create(db_file).await.expect("Failed to create db file");
    }

    let pool = SqlitePoolOptions::new().max_connections(5).connect(db_url).await?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            content_path TEXT NOT NULL
        )
        "#,
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS programs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            desc TEXT NOT NULL,
            file_path TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            channel_id INTEGER NOT NULL,
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}