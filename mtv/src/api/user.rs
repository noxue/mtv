use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use mtv_srv as srv;

use crate::{middleware::{Me, AppId}, utils::res::Res};

use super::PageQuery;

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub code: String,
    pub login_type: String, // mp or  weapp
}

pub async fn login(data: web::Json<LoginInfo>, appid:AppId) -> actix_web::Result<impl Responder> {
    log::debug!("login: {:?}", data);
    let LoginInfo {
        code,
        login_type,
    } = data.into_inner();

    let appid = appid.get_appid()?;


    let token = srv::user::login(&appid, &code, &login_type).await?;
    log::debug!("login res: {:?}", token);

    let mut res = Res::new();
    res.set_data(token);

    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct LoginPhone {
    pub phone: String,
    pub password: String,
}

pub async fn login_phone(data: web::Json<LoginPhone>) -> actix_web::Result<impl Responder> {
    let data = data.into_inner();
    let token = srv::user::login_phone(&data.phone, &data.password).await?;
    let mut res = Res::new();
    res.set_data(token);
    Ok(res)
}

// 设置手机号密码
pub async fn set_phone_password(
    me: Me,
    data: web::Json<LoginPhone>,
) -> actix_web::Result<impl Responder> {
    let data = data.into_inner();
    log::debug!("set_phone_password: {:?}", data);
    srv::user::set_phone_and_password(me.id, &data.phone, &data.password).await?;
    let mut res = Res::new();
    res.set_data("");
    Ok(res)
}

pub async fn me(me: Me) -> actix_web::Result<impl Responder> {
    let user = mtv_srv::user::get(me.id).await?;

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
    mtv_srv::user::set_channel(me.id, &data.channel).await?;

    let mut res = Res::new();
    res.set_data("");
    Ok(res)
}

// 分页列出用户
pub async fn users(query: web::Query<PageQuery>) -> actix_web::Result<impl Responder> {
    let PageQuery { page, size } = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let users = mtv_srv::user::list(page, size).await?;

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
    let users = mtv_srv::user::list_by_channel(&channel, page, size).await?;

    let mut res = Res::new();
    res.set_data(users);
    Ok(res)
}

// 列出我的追剧列表
pub async fn follows(me: Me) -> actix_web::Result<impl Responder> {
    let follows = mtv_srv::movie::follow_list(me.id).await?;

    let mut res = Res::new();
    res.set_data(follows);
    Ok(res)
}

// 最近观看
pub async fn recents(me: Me) -> actix_web::Result<impl Responder> {
    let recent_view = mtv_srv::movie::recent_view(me.id).await?;

    let mut res = Res::new();
    res.set_data(recent_view);
    Ok(res)
}

// 充值记录列表
pub async fn recharges(me: Me, query: web::Query<PageQuery>) -> actix_web::Result<impl Responder> {
    let PageQuery { page, size } = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let v = mtv_srv::order::recharge_record_list(me.id, page, size).await?;
    let mut res = Res::new();
    res.set_data(v);
    Ok(res)
}

// 消费记录
pub async fn consumes(me: Me, query: web::Query<PageQuery>) -> actix_web::Result<impl Responder> {
    let PageQuery { page, size } = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let v = mtv_srv::order::consume_record_list(me.id, page, size).await?;
    let mut res = Res::new();
    res.set_data(v);
    Ok(res)
}
