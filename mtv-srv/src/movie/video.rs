use crate::Result;
use crate::SrvError::NotEnoughGold;
use chrono::Local;
use mtv_dao::{
    movie::video::{UpdateVideo, Video, VideoList},
    Db,
};
use serde::Serialize;

pub async fn add(
    movie_id: i32,
    name: String,
    video: String,
    price: i32,
    status: i32,
    rank: i32,
) -> Result<Video> {
    let conn = Db::get_conn().await;
    let v = mtv_dao::movie::video::add(&conn, movie_id, name, video, price, status, rank).await?;
    Ok(v)
}

pub async fn list(movie_id: i32) -> Result<Vec<VideoList>> {
    let conn = Db::get_conn().await;
    let v = mtv_dao::movie::video::list(&conn, movie_id).await?;
    Ok(v)
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoWithLike {
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
    pub is_liked: bool,
    pub is_followed: bool,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}
pub async fn get(video_id: i32, user_id: i32, is_admin: bool) -> Result<VideoWithLike> {
    let conn = Db::get_conn().await;
    let v = mtv_dao::movie::video::get(&conn, video_id).await?;

    if v.is_none() {
        return Err("视频不存在".into());
    }
    let v = v.unwrap();

    // 是否点赞
    let is_liked = mtv_dao::movie::video::is_liked(&conn, user_id, video_id).await?;
    // 是否追剧
    let is_followed = mtv_dao::movie::video::is_followed(&conn, user_id, v.movie_id).await?;

    let v = VideoWithLike {
        id: v.id,
        movie_id: v.movie_id,
        name: v.name,
        video: v.video,
        price: v.price,
        status: v.status,
        likes: v.likes,
        share: v.share,
        view: v.view,
        rank: v.rank,
        is_liked,
        is_followed,
        create_time: v.create_time,
        update_time: v.update_time,
    };

    // 不是管理员，就需要扣除金币，增加播放量
    if !is_admin {
        mtv_dao::movie::add_view(&conn, v.movie_id, v.id).await?;
        mtv_dao::movie::video::add_view_record(&conn, user_id, v.movie_id, video_id).await?;
        if v.price > 0 {
            // 已经付费过，直接返回
            if mtv_dao::order::consume_record_by_user_id_and_video_id(&conn, user_id, video_id)
                .await?
            {
                return Ok(v);
            }

            let user = mtv_dao::user::get(&conn, user_id).await?;

            let mut price = 0;

            // 如果会员到期了，就扣费
            if user.vip_expire_time < chrono::Local::now() {
                if let Err(e) = mtv_dao::user::update_score(&conn, user_id, -v.price).await {
                    if e.to_string().contains("金币不足") {
                        return Err(NotEnoughGold.into());
                    }
                    return Err(e.into());
                };
                price = v.price; // 扣了多少记录下来
            }

            if v.price > 0 {
                // 添加消费记录
                mtv_dao::order::add_consume_record(
                    &conn,
                    user_id,
                    v.movie_id,
                    v.id,
                    price,
                    "观看视频".to_string(),
                )
                .await?;
            }
        }
    }

    Ok(v)
}

pub async fn update(video_id: i32, update_video: &UpdateVideo) -> Result<Video> {
    let conn = Db::get_conn().await;
    let v = mtv_dao::movie::video::update(&conn, video_id, update_video).await?;
    Ok(v)
}

// 删除
pub async fn delete(video_id: i32) -> Result<()> {
    let conn = Db::get_conn().await;
    mtv_dao::movie::video::delete(&conn, video_id).await?;
    Ok(())
}
