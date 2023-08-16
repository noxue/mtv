use mtv_dao::{
    movie::{
        video::{Video, VideoList},
        Movie, UpdateMovie,
    },
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

pub async fn list(movie_id: i32) -> Result<Vec<VideoList>> {
    let conn = Db::get_conn();
    let v = mtv_dao::movie::video::list(&conn, movie_id).await?;
    Ok(v)
}

pub async fn get(video_id: i32, user_id: i32, is_admin: bool) -> Result<Video> {
    let conn = Db::get_conn();
    let v = mtv_dao::movie::video::get(&conn, video_id).await?;

    if v.is_none() {
        return Err("视频不存在".into());
    }
    let v = v.unwrap();

    // 不是管理员，就需要扣除金币，增加播放量
    if !is_admin {
        mtv_dao::movie::add_view(&conn, v.movie_id, v.id).await?;
        if v.price > 0 {

            // 已经付费过，直接返回
            if mtv_dao::order::consume_record_by_user_id_and_video_id(&conn, user_id, video_id).await?{
                return Ok(v);
            }

            let user = mtv_dao::user::get(&conn, user_id).await?;

            let mut price = 0;

            // 如果会员到期了，就扣费
            if user.vip_expire_time < chrono::Local::now() {
                mtv_dao::user::update_score(&conn, user_id, -v.price).await?;
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
