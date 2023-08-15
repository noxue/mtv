use actix_web::Responder;
use actix_web::Result;

// 添加影片
pub async fn add() -> Result<impl Responder> {
    Ok("")
}

// 添加单集视频
pub async fn add_video() -> Result<impl Responder> {
    Ok("")
}

// 列出所有短剧，按照点赞量，观看量，追剧量 排名
pub async fn list() -> Result<impl Responder> {
    Ok("")
}

// 获取单个短剧的信息，封面，标题，总集数，简介，点赞量，观看量，追剧量
pub async fn get() -> Result<impl Responder> {
    Ok("")
}

// 列出短剧所有集的信息，支持分页
pub async fn list_video() -> Result<impl Responder> {
    Ok("")
}

// 查看单集短剧的信息
pub async fn get_video(video_id: i32) -> Result<impl Responder> {
    Ok("")
}

// 点赞短剧,取消点赞
pub async fn like(video_id: i32, cancel: bool) -> Result<impl Responder> {
    Ok("")
}

// 追剧，取消追剧
pub async fn follow(video_id: i32) -> Result<impl Responder> {
    Ok("")
}
