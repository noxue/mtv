use chrono::Local;
use mtv_dao::{order::*, Db, Page};
use serde::Serialize;
use crate::Result;

// // 添加订单
// pub async fn add(user_id: i32, goods_id: i32) -> Result<Order> {
//     let conn = Db::get_conn();

//     // 获取商品信息
//     let goods = mtv_dao::goods::get(&conn, goods_id).await?;
//     if goods.is_none() {
//         return Err("商品不存在".into());
//     }

//     let goods = goods.unwrap();

//     // 创建订单
//     let order = mtv_dao::order::add(
//         &conn,
//         goods_id,
//         user_id,
//         goods.price,
//         uuid::Uuid::new_v4()
//             .to_string()
//             .replace("-", "")
//             .to_string(),
//         goods.name,
//     )
//     .await
//     .map_err(|e| {
//         log::error!("创建订单出错:{:?}", e);
//         "创建订单出错"
//     })?;

//     Ok(order)
// }

// 添加商品
pub async fn add(
    name: String,
    price: i32,
    description: String,
    score: i32,
    is_hot: bool,
    is_vip: bool,
    expire_type: i32,
    expire_count: i32,
) -> Result<bool> {
    let conn = Db::get_conn();

    // 创建商品
    let ok = mtv_dao::goods::add(
        &conn,
        name,
        price,
        description,
        score,
        is_hot,
        is_vip,
        expire_type,
        expire_count,
    )
    .await
    .map_err(|e| {
        log::error!("创建商品出错:{:?}", e);
        "创建商品出错"
    })?;

    Ok(ok)
}

// 商品列表
pub async fn list() -> Result<impl Serialize> {
    let conn = Db::get_conn();
    let goods = mtv_dao::goods::list(&conn).await?;
    Ok(goods)
}

// 根据商品id查询商品
pub async fn get(id: i32) -> Result<impl Serialize> {
    let conn = Db::get_conn();
    let goods = mtv_dao::goods::get(&conn, id).await?;
    Ok(goods)
}

// 删除商品
pub async fn delete(id: i32) -> Result<()> {
    let conn = Db::get_conn();
    mtv_dao::goods::delete(&conn, id).await?;
    Ok(())
}

// 更新商品
pub async fn update(
    id: i32,
    name: String,
    price: i32,
    description: String,
    score: i32,
    is_hot: bool,
    is_vip: bool,
    expire_type: i32,
    expire_count: i32,
) -> Result<()> {
    let conn = Db::get_conn();
    mtv_dao::goods::update(
        &conn,
        id,
        name,
        price,
        description,
        score,
        is_hot,
        is_vip,
        expire_type,
        expire_count,
    )
    .await?;
    Ok(())
}
