use sqlx::Row;
use sqlx::{Pool, Sqlite};
use crate::utils::player::play_file;

pub async fn add_program(pool: &Pool<Sqlite>, name: &str, desc: &str, path: &str, start_time: &str, end_time: &str, channel_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO programs (name, desc, file_path, start_time, end_time, channel_id) 
        VALUES (?, ?, ?, ?, ?, ?)"#
    ).bind(name).bind(desc).bind(path).bind(start_time).bind(end_time).bind(channel_id).execute(pool).await?;
    
    println!("The following program has been added:\nName: {}\nDesc: {}\nPath: {}\nStart Time: {}\nEnd Time: {}\nChannel ID: {}", name, desc, path, start_time, end_time, channel_id);
   
    Ok(())
}

pub async fn enable_program(pool: &Pool<Sqlite>, id: i64, enable: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE programs SET enable = ? WHERE id = ?").bind(enable).bind(id).execute(pool).await?;  
    
    println!("Program ID '{}' has been {}", id, if enable { "enabled" } else { "disabled" });
   
    Ok(())
}

pub async fn play_program(pool: &Pool<Sqlite>, id: i64) -> Result<(), sqlx::Error> {
    let row = sqlx::query(
        r#"SELECT file_path FROM programs WHERE id = ?"#,
    ).bind(id).fetch_one(pool).await?;

    let program_path: String = row.get("file_path");
    
    println!("Now playing '{}'", program_path);

    if let Err(e) = play_file(&program_path).await {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

pub async fn list_programs(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT 
            p.id,
            p.name,
            p.desc,
            p.start_time,
            p.end_time,
            p.file_path,
            c.id as channel_id,
            c.name as channel_name
        FROM programs p
        JOIN channels c ON p.channel_id = c.id
        "#
    )
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        println!("No Programs found.");
    } else {
        for row in rows {
            let id: i64 = row.get("id");
            let name: String = row.get("name");
            let desc: String = row.get("desc");
            let start_time: String = row.get("start_time");
            let end_time: String = row.get("end_time");
            let file_path: String = row.get("file_path");
            let channel_name: String = row.get("channel_name");

            println!(
                "[{}] ({}) {} -> {}\n{} -> {}\n{}\n",
                id, channel_name, name, desc, start_time, end_time, file_path
            );
        }
    }

    Ok(())
}
