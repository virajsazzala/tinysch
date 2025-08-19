use sqlx::{Pool, Sqlite};
use crate::model::Channel;

pub async fn add_channel(pool: &Pool<Sqlite>, name: &str, path: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO channels (name, content_path) VALUES (?, ?)").bind(name).bind(path).execute(pool).await?;
    
    println!("Channel '{}' added with path '{}'", name, path);
   
    Ok(())
}

pub async fn list_channels(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query_as::<_, Channel>("SELECT * from channels").fetch_all(pool).await?;

    if rows.is_empty() {
        println!("No Channels found.");
    } else {
        for ch in rows {
            println!("[{}] {} -> {}", ch.id, ch.name, ch.content_path);
        }
    }

    Ok(())
}