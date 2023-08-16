pub mod video;

use mtv_dao::{
    movie::{Movie, UpdateMovie},
    Db, Page,
};
use serde::{Deserialize, Serialize};

use crate::Result;

pub async fn add(
    name: String,
    cover: String,
    description: String,
    tags: Vec<String>,
    price_total: i32,
    vlikes: i32,
) -> Result<Movie> {
    let conn = Db::get_conn();
    let movie =
        mtv_dao::movie::add(&conn, name, cover, description, tags, price_total, vlikes).await?;
    Ok(movie)
}

/// 获取
pub async fn get(id: i32) -> Result<Movie> {
    let conn = Db::get_conn();
    let movie = mtv_dao::movie::get(&conn, id).await?;
    Ok(movie)
}

/// 更新
pub async fn update(id: i32, update_movie: &UpdateMovie) -> Result<Movie> {
    let conn = Db::get_conn();
    let m = mtv_dao::movie::update(&conn, id, update_movie).await?;
    Ok(m)
}

/// 删除
pub async fn delete(id: i32) -> Result<()> {
    let conn = Db::get_conn();
    mtv_dao::movie::delete(&conn, id).await?;
    Ok(())
}

// 列表
pub async fn list(
    page: i64,
    page_size: i64,
    desc: Option<&String>,
    tags: Option<&String>,
    hot: Option<&String>,
    top: Option<&String>,
    like: Option<&String>,
    view: Option<&String>,
    update_time: Option<&String>,
    price: Option<&String>,
) -> Result<impl Serialize> {
    let conn = Db::get_conn();

    if tags.is_some() {
        let tags = tags.unwrap().split(",").map(|s| s.to_string()).collect();
        let movies = mtv_dao::movie::list_tags(&conn, tags, desc.is_some(), page, page_size).await?;
        return Ok(movies);
    }

    if hot.is_some() {
        let movies = mtv_dao::movie::list_hot(&conn, desc.is_some(), page, page_size).await?;
        return Ok(movies);
    }
    if top.is_some() {
        let movies = mtv_dao::movie::list_top(&conn, desc.is_some(), page, page_size).await?;
        return Ok(movies);
    }

    if like.is_some() {
        let movies = mtv_dao::movie::list_like(&conn, desc.is_some(), page, page_size).await?;
        return Ok(movies);
    }

    if view.is_some() {
        let movies = mtv_dao::movie::list_view(&conn, desc.is_some(), page, page_size).await?;
        return Ok(movies);
    }

    if update_time.is_some() {
        let movies = mtv_dao::movie::list_update_time(&conn, desc.is_some(), page, page_size).await?;
        return Ok(movies);
    }

    // if price.is_some(){
    //     let movies = mtv_dao::movie::list_price(&conn, desc.is_some(), page, page_size).await?;
    //     return Ok(movies);
    // }

    let movies = mtv_dao::movie::list(&conn, desc.is_some(), page, page_size).await?;
    Ok(movies)
}
