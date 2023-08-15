use chrono::Local;
use serde::Serialize;
use sqlrs::{Conn, Db, Table};

#[derive(Debug, Clone, Serialize, Table)]
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub cover: String,
    pub total: i32,
    pub description: String,
    pub is_top: bool,
    pub is_hot: bool,
    pub tags: Vec<String>,
    pub price_total: i32,
    pub price_single: i32,
    pub is_show: bool,
    pub view: i32,
    pub vlikes: i32,
    pub is_finish: bool,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct MoviePart {
    pub id: i32,
    pub movie_id: i32,
    pub name: String,
    pub video: String,
    pub price: i32,
    pub status: i32,
    pub likes: i32,
    pub share: i32,
    pub view: i32,
    pub rank: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}
