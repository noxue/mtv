pub mod movie;
pub mod order;
pub mod pay;
pub mod user;

use actix_web::{web, Scope};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    page: Option<i64>,
    size: Option<i64>,
}

async fn down_up() -> &'static str {
    mtv_srv::user::down_up().await.unwrap();
    "xx"
}

async fn test() -> &'static str {
    ""
}

pub fn api() -> Scope {
    web::scope("/api")
        .route("/down_up", web::get().to(down_up))
        .service(
            web::scope("/users")
                .route("", web::get().to(user::users))
                // 列出指定渠道的用户列表
                .route("/channel/{channel}", web::get().to(user::users_by_channel))
                .route("/login", web::post().to(user::login))
                .route("/me", web::get().to(user::me))
                // 设置渠道来源
                .route("/channel", web::post().to(user::set_channel))
                // 最近观看
                .route("/recents", web::get().to(user::recents))
                // 追剧列表
                .route("/follows", web::get().to(user::follows))
                // 充值记录
                .route("/recharges", web::get().to(user::recharges))
                // 消费记录
                .route("/consumes", web::get().to(user::consumes)),
        )
        // movies
        .service(
            web::scope("/movies")
                // 短剧列表，根据点赞量，播放量，更新时间，付费用户量
                .route("", web::get().to(movie::list))
                // 添加短剧
                .route("", web::post().to(movie::add))
                // 获取短剧
                .route("/{id}", web::get().to(movie::get))
                // 更新短剧
                .route("/{id}", web::put().to(movie::update))
                // 删除短剧
                .route("/{id}", web::delete().to(movie::delete))
                // 添加影片
                .route("/{movie_id}/videos", web::post().to(movie::add_video))
                // 影片列表
                .route("/{movie_id}/videos", web::get().to(movie::list_video))
                // 更新影片
                .route(
                    "/videos/{video_id}",
                    web::put().to(movie::update_video),
                )
                // 获得影片，在其中执行扣除金币的操作，如果成功返回影片信息，如果扣费失败 返回对应错误，比如金币不足
                .route(
                    "/videos/{video_id}",
                    web::get().to(movie::get_video),
                )
                // 删除影片
                .route(
                    "/videos/{video_id}",
                    web::delete().to(movie::delete_video),
                )
                // 追剧
                .route("/{movie_id}/follow", web::post().to(movie::follow))
                // 点赞
                .route("/like", web::post().to(movie::like)),
        )
        // orders
        .service(
            web::scope("/orders")
                // 创建订单
                .route("", web::post().to(order::create))
                // 订单列表
                .route("", web::get().to(order::list))
                // 订单详情
                .route("/{id}", web::get().to(order::get))
                // 支付签名
                .route("/{id}/pay", web::post().to(pay::sign))
                // 支付回调
                .route("/{id}/pay/callback", web::post().to(pay::callback))
                // 检测支付情况
                .route("/{id}/pay/check", web::get().to(pay::check))
                // 所有充值记录
                .route("/recharges", web::get().to(order::recharges))
                // 所有消费记录
                .route("/consumes", web::get().to(order::consumes)),
        )
}
