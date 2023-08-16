use chrono::Local;
use serde::Serialize;
use sqlrs::{Conn, Table};

#[derive(Debug, Clone, Serialize, Table)]
pub struct Video {
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

pub async fn add(
    conn: &Conn,
    movie_id: i32,
    name: String,
    video: String,
    price: i32,
    status: i32,
    rank: i32,
) -> anyhow::Result<Video> {
    let row = conn
        .query_one(
            r#" insert into videos (movie_id, name, video, price, status, rank) values ($1, $2, $3, $4, $5, $6) returning * "#,
            &[&movie_id, &name, &video, &price, &status, &rank],
        )
        .await?;
    Ok(row.try_into()?)
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct VideoList {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub rank: i32,
}

pub async fn list(conn: &Conn, movie_id: i32) -> anyhow::Result<Vec<VideoList>> {
    let rows = conn
        .query(
            r#" select id, name, price, rank from videos where movie_id = $1 order by rank asc, id asc"#,
            &[&movie_id],
        )
        .await?;
    Ok(rows.iter().map(|row| row.try_into().unwrap()).collect())
}

pub async fn get(conn: &Conn, video_id: i32) -> anyhow::Result<Option<Video>> {
    let row = conn
        .query_opt(r#" select * from videos where id = $1 "#, &[&video_id])
        .await?;

    if let Some(row) = row {
        Ok(Some(row.try_into()?))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct ViewRecord {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub movie_part_id: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct FollowRecord {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}
