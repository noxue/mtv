use mtv_dao::{
    movie::{video::Video, Movie, UpdateMovie},
    Db, Page,
};
use serde::{Deserialize, Serialize};

use crate::Result;

pub async fn add(
    movie_id: i32,
    name: String,
    video: String,
    price: i32,
    status: i32,
    rank: i32,
) -> Result<Video> {
    let conn = Db::get_conn();
    let v = mtv_dao::movie::video::add(&conn, movie_id, name, video, price, status, rank).await?;
    Ok(v)
}
