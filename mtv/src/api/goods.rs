use actix_web::web;
use actix_web::Responder;
use actix_web::Result;
use serde::Deserialize;

use crate::middleware::Me;
use crate::utils::res::Res;

use super::PageQuery;

// #[derive(Debug, Deserialize)]
// pub struct CreateOrder {
//     pub goods_id: i32,
// }

// // 创建订单
// pub async fn create(me: Me, data: web::Json<CreateOrder>) -> Result<impl Responder> {
//     let ret = mtv_srv::order::add(me.id, data.goods_id).await?;
//     let mut res = Res::new();
//     res.set_data(ret);
//     Ok(res)
// }



#[derive(Debug, Deserialize)]
pub struct CreateGoods {
    pub name:String,
    pub price: i32,
    pub description: String,
    pub score: i32,
    pub is_hot:bool,
    pub is_vip: bool,
    pub expire_type: i32, // 会员过期类型 0:月 1:季 2:年
    pub expire_count: i32, // 会员过期数量,上面是单位，这个是数量
}

// 创建商品
pub async fn create(data: web::Json<CreateGoods>) -> Result<impl Responder> {
    let ret = mtv_srv::goods::add(
        data.name.clone(),
        data.price,
        data.description.clone(),
        data.score,
        data.is_hot,
        data.is_vip,
        data.expire_type,
        data.expire_count,
    ).await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}

// 商品列表
pub async fn list() -> Result<impl Responder> {
    let ret = mtv_srv::goods::list().await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}

// 根据商品id查询商品
pub async fn get(id: web::Path<i32>) -> Result<impl Responder> {
    let id = id.into_inner();
    let ret = mtv_srv::goods::get(id).await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}

// 删除商品
pub async fn delete(id: web::Path<i32>) -> Result<impl Responder> {
    let id = id.into_inner();
    let ret = mtv_srv::goods::delete(id).await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}

// 更新商品
pub async fn update(id: web::Path<i32>, data: web::Json<CreateGoods>) -> Result<impl Responder> {
    let id = id.into_inner();
    let ret = mtv_srv::goods::update(
        id,
        data.name.clone(),
        data.price,
        data.description.clone(),
        data.score,
        data.is_hot,
        data.is_vip,
        data.expire_type,
        data.expire_count,
    ).await?;
    let mut res = Res::new();
    res.set_data(ret);
    Ok(res)
}