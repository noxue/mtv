use actix_web::Result;
use actix_web::{web, Responder};
use mtv_dao::movie::video::UpdateVideo;
use mtv_dao::movie::UpdateMovie;
use serde::Deserialize;
use std::collections::HashMap;

use crate::middleware::Me;
use crate::utils::res::Res;

use super::PageQuery;

#[derive(Debug, Deserialize)]
pub struct AddData {
    pub name: String,
    pub cover: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub price_total: Option<i32>,
    pub vlikes: Option<i32>,
}

// 添加影片
pub async fn add(data: web::Json<AddData>) -> Result<impl Responder> {
    let AddData {
        name,
        cover,
        description,
        tags,
        price_total,
        vlikes,
    } = data.into_inner();

    let cover = cover.unwrap_or("".to_string());
    let description = description.unwrap_or("".to_string());
    let tags = tags.unwrap_or(vec![]);
    let price_total = price_total.unwrap_or(0);
    let vlikes = vlikes.unwrap_or(1000);

    let id = mtv_srv::movie::add(name, cover, description, tags, price_total, vlikes).await?;
    let mut res = Res::new();
    res.set_data(id);
    Ok(res)
}

// 获取单个短剧的信息，封面，标题，总集数，简介，点赞量，观看量，追剧量
pub async fn get(id: web::Path<i32>) -> Result<impl Responder> {
    let id = id.into_inner();
    let movie = mtv_srv::movie::get(id).await?;
    let mut res = Res::new();
    res.set_data(movie);
    Ok(res)
}

// 更新短剧
pub async fn update(id: web::Path<i32>, data: web::Json<UpdateMovie>) -> Result<impl Responder> {
    let m = mtv_srv::movie::update(id.into_inner(), &data).await?;
    let mut res = Res::new();
    res.set_data(m);
    Ok(res)
}

// 删除短剧
pub async fn delete(id: web::Path<i32>) -> Result<impl Responder> {
    let id = id.into_inner();
    mtv_srv::movie::delete(id).await?;
    let mut res = Res::new();
    res.set_data(id);
    Ok(res)
}

/// 短剧列表，根据点赞量，播放量，更新时间，付费用户量
/// desc：带了参数就是倒序，没带参数就是顺序
///
/// 下面条件同时只能出现一个
/// tags: 标签，多个标签用逗号分隔
/// hot: 是否热门
/// top: 是否置顶
/// like: 根据点赞量排序
/// view: 根据播放量排序
/// update_time: 根据更新时间排序
/// price: 根据付费用户量排序
pub async fn list(
    page_params: web::Query<PageQuery>,
    data: web::Query<HashMap<String, String>>,
) -> Result<impl Responder> {
    log::info!("page: {:?}", page_params);
    log::info!("data: {:?}", data);
    let data = data.into_inner();
    let desc = data.get("desc");
    let tags = data.get("tags");
    let hot = data.get("hot");
    let top = data.get("top");
    let like = data.get("like");
    let view = data.get("view");
    let update_time = data.get("update_time");
    let price = data.get("price");
    let movies = mtv_srv::movie::list(
        page_params.page.unwrap_or(1),
        page_params.size.unwrap_or(20),
        desc,
        tags,
        hot,
        top,
        like,
        view,
        update_time,
        price,
    )
    .await?;

    let mut res = Res::new();
    res.set_data(movies);
    Ok(res)
}

// 短剧的单集视频信息
#[derive(Debug, Deserialize)]
pub struct AddVideoData {
    pub name: String,
    pub video: String,
    pub price: Option<i32>,
    pub rank: Option<i32>,   // 排序,数字越大越靠后
    pub status: Option<i32>, // 状态 0:下架 1:上架，默认上架
}
// 添加单集视频
pub async fn add_video(
    movie_id: web::Path<i32>,
    data: web::Json<AddVideoData>,
) -> Result<impl Responder> {
    let movie_id = movie_id.into_inner();
    let AddVideoData {
        name,
        video,
        price,
        rank,
        status,
    } = data.into_inner();
    let price = price.unwrap_or(0);
    let status = status.unwrap_or(1);
    let rank = rank.unwrap_or(0);
    let v = mtv_srv::movie::video::add(movie_id, name, video, price, status, rank).await?;
    let mut res = Res::new();
    res.set_data(v);
    Ok(res)
}

// 列出短剧所有集的部分信息，支持分页
pub async fn list_video(movie_id: web::Path<i32>) -> Result<impl Responder> {
    let movie_id = movie_id.into_inner();
    let videos = mtv_srv::movie::video::list(movie_id).await?;
    let mut res = Res::new();
    res.set_data(videos);
    Ok(res)
}

// 更新单集短剧的信息
pub async fn update_video(
    video_id: web::Path<i32>,
    data: web::Json<UpdateVideo>,
) -> Result<impl Responder> {
    let video_id = video_id.into_inner();
    let v = mtv_srv::movie::video::update(video_id, &data).await?;
    let mut res = Res::new();
    res.set_data(v);
    Ok(res)
}

// 查看单集短剧的信息
pub async fn get_video(video_id: web::Path<i32>, me: Me) -> Result<impl Responder> {
    let v = mtv_srv::movie::video::get(video_id.into_inner(), me.id, me.is_admin()).await?;
    let mut res = Res::new();
    res.set_data(v);
    Ok(res)
}

// 删除单集短剧
pub async fn delete_video(video_id: web::Path<i32>) -> Result<impl Responder> {
    let video_id = video_id.into_inner();
    mtv_srv::movie::video::delete(video_id).await?;
    let mut res = Res::new();
    res.set_data(video_id);
    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct LikeData {
    movie_id: i32,
    video_id: i32,
    pub like: bool,
}

// 点赞短剧,取消点赞
pub async fn like(me: Me, data: web::Json<LikeData>) -> Result<impl Responder> {
    let LikeData {
        movie_id,
        video_id,
        like,
    } = data.into_inner();
    mtv_srv::movie::like(me.id, movie_id, video_id, like).await?;
    let mut res = Res::new();
    res.set_data("");
    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct FollowData {
    movie_id: i32,
    pub follow: bool,
}

// 追剧，取消追剧
pub async fn follow(me: Me, data: web::Json<FollowData>) -> Result<impl Responder> {
    let FollowData { movie_id, follow } = data.into_inner();
    mtv_srv::movie::follow(me.id, movie_id, follow).await?;
    let mut res = Res::new();
    res.set_data("");
    Ok(res)
}
