use actix_web::{web, HttpResponse, Responder};
use mtv_config::{DataConfig, CONFIG};
use serde::Deserialize;

use mtv_srv as srv;

use crate::{middleware::Me, utils::res::Res};

use super::PageQuery;

// 生成上传oss的token
pub async fn oss_token(me: Me) -> actix_web::Result<impl Responder> {
    let uid = me.id;

    let api_host = &CONFIG.oss_api_host;
    let access_key_id = &CONFIG.oss_access_key_id;
    let access_key_secret = &CONFIG.oss_access_key_secret;
    let host = &CONFIG.oss_host;
    let callback_url = &CONFIG.oss_callback_url;
    let bucket_name = &CONFIG.oss_bucket_name;
    let expire_time = CONFIG.oss_expire_time;

    let filename = format!(
        "file/u-{}-{}",
        uid,
        uuid::Uuid::new_v4().to_string().replace("-", "")
    );

    let token = mtv_srv::utils::oss::get_policy_token(
        api_host,
        access_key_id,
        access_key_secret,
        &filename,
        host,
        callback_url,
        bucket_name,
        expire_time,
    )
    .await?;

    let mut res = Res::<serde_json::Value>::new();
    res.set_data(serde_json::from_str(token.as_str()).unwrap());

    Ok(res)
}
