use actix_web::web;
use actix_web::Responder;
use actix_web::Result;
use serde::Deserialize;

use crate::middleware::Me;
use crate::utils::res::Res;

use super::PageQuery;

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub goods_id: i32,
}

// 创建订单
pub async fn create(me: Me, data: web::Json<CreateOrder>) -> Result<impl Responder> {
    let ret = mtv_srv::order::add(me.id, data.goods_id).await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}

// 订单列表
pub async fn list(me: Me, query: web::Query<PageQuery>) -> Result<impl Responder> {
    let PageQuery { page, size } = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let ret = mtv_srv::order::list_by_user_id(me.id, page, size).await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}

// 订单详情
pub async fn get(order_no: web::Path<String>) -> Result<impl Responder> {
    let order_no = order_no.into_inner();
    let ret = mtv_srv::order::get(&order_no).await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}

// 充值记录列表
pub async fn recharges(me: Me, query: web::Query<PageQuery>) -> Result<impl Responder> {
    let PageQuery { page, size } = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let v = mtv_srv::order::recharge_record_list(me.id, page, size).await?;
    let mut res = Res::new();
    res.set_data(v);
    Ok(res)
}

// 消费记录
pub async fn consumes(me: Me, query: web::Query<PageQuery>) -> Result<impl Responder> {
    let PageQuery { page, size } = query.into_inner();
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let v = mtv_srv::order::consume_record_list(me.id, page, size).await?;
    let mut res = Res::new();
    res.set_data(v);
    Ok(res)
}
