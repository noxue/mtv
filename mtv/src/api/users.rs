use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use mtv_srv as srv;

use crate::{middleware::Me, utils::res::Res};

use super::PageQuery;

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub code: String,
    pub login_type: String, // mp or  weapp
}

pub async fn login(data: web::Json<LoginInfo>) -> actix_web::Result<impl Responder> {
    log::debug!("login: {:?}", data);
    let LoginInfo { code, login_type } = data.into_inner();

    let token = srv::users::login(&code, &login_type).await?;
    log::debug!("login res: {:?}", token);

    let mut res = Res::new();
    res.set_data(token);

    Ok(res)
}

pub async fn me(me: Me) -> actix_web::Result<impl Responder> {
    let user = mtv_srv::users::get(me.id).await?;

    let mut res = Res::new();
    res.set_data(user);
    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct SetChannel {
    pub channel: String,
}

// 设置渠道
pub async fn set_channel(me: Me, data: web::Json<SetChannel>) -> actix_web::Result<impl Responder> {
    let data = data.into_inner();
    mtv_srv::users::set_channel(me.id, &data.channel).await?;

    let mut res = Res::new();
    res.set_data("");
    Ok(res)
}

// 分页列出用户
pub async fn users(query: web::Query<PageQuery>) -> actix_web::Result<impl Responder> {
    let PageQuery { page, size } = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let users = mtv_srv::users::list(page, size).await?;

    let mut res = Res::new();
    res.set_data(users);
    Ok(res)
}

// 分页列出指定渠道的用户
pub async fn users_by_channel(
    query: web::Query<PageQuery>,
    channel: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let (PageQuery { page, size }) = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let users = mtv_srv::users::list_by_channel(&channel, page, size).await?;

    let mut res = Res::new();
    res.set_data(users);
    Ok(res)
}

// 列出我的追剧列表

// 列出浏览历史
