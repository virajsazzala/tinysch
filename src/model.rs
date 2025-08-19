use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub content_path: String,
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Program {
    pub id: i64,
    pub name: String,
    pub desc: String,
    pub file_path: String,
    pub start_time: String,
    pub end_time: String,
    pub channel_id: i64,
    pub enable: bool,
}
