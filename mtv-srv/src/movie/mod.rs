pub mod video;

use mtv_dao::{
    movie::{Movie, UpdateMovie},
    transaction, Db, Page,
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
        let movies =
            mtv_dao::movie::list_tags(&conn, tags, desc.is_some(), page, page_size).await?;
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
        let movies =
            mtv_dao::movie::list_update_time(&conn, desc.is_some(), page, page_size).await?;
        return Ok(movies);
    }

    // if price.is_some(){
    //     let movies = mtv_dao::movie::list_price(&conn, desc.is_some(), page, page_size).await?;
    //     return Ok(movies);
    // }

    let movies = mtv_dao::movie::list(&conn, desc.is_some(), page, page_size).await?;
    Ok(movies)
}

// 点赞，同时更新movie 和 vedio
pub async fn like(user_id: i32, movie_id: i32, video_id: i32, is_like: bool) -> Result<()> {
    let mut conn = Db::get_conn();
    let likes = if is_like { 1 } else { -1 };

    let is_liked = mtv_dao::movie::video::is_liked(&conn, user_id, video_id).await?;

    log::debug!("is_liked: {}", is_liked);
    // 已经点过赞，就不再点赞了，或者已经取消点赞，就不再取消点赞了
    if is_liked == is_like {
        log::debug!("已经点过赞，就不再点赞了，或者已经取消点赞，就不再取消点赞了");
        return Ok(());
    }
    log::debug!("is_like: {}", is_like);

    transaction! {conn,{
        mtv_dao::movie::update_likes(&conn, movie_id, likes)
            .await
            .map_err(|e| {
                log::error!("更新movie点赞量失败: {:?}", e);
                "点赞失败"
            })?;

        mtv_dao::movie::video::update_likes(&conn, video_id, likes)
            .await
            .map_err(|e| {
                log::error!("更新video点赞量失败: {:?}", e);
                "点赞失败"
            })?;

        if is_like {
            mtv_dao::movie::video::add_likes_record(&conn, user_id, movie_id, video_id)
                .await
                .map_err(|e| {
                    log::error!("添加点赞记录失败: {:?}", e);
                    "点赞失败"
                })?;
        } else {
            mtv_dao::movie::video::delete_likes_record(&conn, user_id, video_id)
                .await
                .map_err(|e| {
                    log::error!("删除点赞记录失败: {:?}", e);
                    "取消点赞失败"
                })?;
        }

    }}

    Ok(())
}

// 追剧
pub async fn follow(user_id: i32, movie_id: i32, is_follow: bool) -> Result<()> {
    let conn = Db::get_conn();

    let is_followed = mtv_dao::movie::video::is_followed(&conn, user_id, movie_id).await?;

    log::debug!("is_followed: {}", is_followed);
    // 已经点过赞，就不再点赞了，或者已经取消点赞，就不再取消点赞了
    if is_followed == is_follow {
        log::debug!("已经追剧，就不再追剧了，或者已经取消追剧，就不再取消追剧了");
        return Ok(());
    }
    log::debug!("is_follow: {}", is_follow);

    if is_follow {
        mtv_dao::movie::video::add_follow_record(&conn, user_id, movie_id)
            .await
            .map_err(|e| {
                log::error!("添加追剧记录失败: {:?}", e);
                "追剧失败"
            })?;
    } else {
        mtv_dao::movie::video::delete_follow_record(&conn, user_id, movie_id)
            .await
            .map_err(|e| {
                log::error!("删除追剧记录失败: {:?}", e);
                "取消追剧失败"
            })?;
    }

    Ok(())
}

// 最近观看
pub async fn recent_view(user_id: i32) -> Result<impl Serialize> {
    let conn = Db::get_conn();
    let movies = mtv_dao::movie::video::recent_view(&conn, user_id).await?;
    Ok(movies)
}

// 追剧列表
pub async fn follow_list(user_id: i32) -> Result<impl Serialize> {
    let conn = Db::get_conn();
    let movies = mtv_dao::movie::video::follow_list(&conn, user_id).await?;
    Ok(movies)
}
