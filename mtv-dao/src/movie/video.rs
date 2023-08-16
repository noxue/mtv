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

/*
pub async fn add(
    conn: &Conn,
    name: String,
    cover: String,
    description: String,
    tags: Vec<String>,
    price_total: i32,
    vlikes: i32,
) -> anyhow::Result<Movie> {
    let row = conn
        .query_one(
            r#" insert into movies (name, cover, description, tags, price_total, vlikes) values ($1, $2, $3, $4, $5, $6) returning * "#,
            &[&name, &cover, &description, &tags, &price_total, &vlikes],
        )
        .await.map_err(|e|{
            if e.to_string().contains("duplicate key value violates unique constraint \"movies_name_uindex\""){
                anyhow::anyhow!("电影名已存在")
            }else{
                anyhow::anyhow!("{}", e)
            }
        })?;
    Ok(row.try_into()?)
}
*/
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
