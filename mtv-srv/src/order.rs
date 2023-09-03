use chrono::Local;
use mtv_dao::{order::*, Db, Page};
use serde::Serialize;
use wxpay::WxPayNotify;

use crate::{
    pay::PAY,
    utils::{self},
    Result,
};

// 添加订单
pub async fn add(user_id: i32, goods_id: i32) -> Result<Order> {
    let conn = Db::get_conn().await;

    // 获取商品信息
    let goods = mtv_dao::goods::get(&conn, goods_id).await?;
    if goods.is_none() {
        return Err("商品不存在".into());
    }

    let goods = goods.unwrap();

    // 创建订单
    let order = mtv_dao::order::add(
        &conn,
        goods_id,
        user_id,
        goods.price,
        uuid::Uuid::new_v4()
            .to_string()
            .replace("-", "")
            .to_string(),
        goods.name,
    )
    .await
    .map_err(|e| {
        log::error!("创建订单出错:{:?}", e);
        "创建订单出错"
    })?;

    Ok(order)
}

// 列出指定用户订单列表
pub async fn list_by_user_id(user_id: i32, page: i64, size: i64) -> Result<Page<Vec<Order>>> {
    let conn = Db::get_conn().await;
    let page = mtv_dao::order::list_by_user_id(&conn, user_id, page, size).await?;
    Ok(page)
}

// 查看订单支付情况
pub async fn check(order_no: &str) -> Result<i32> {
    let conn = Db::get_conn().await;
    let status = mtv_dao::order::get_status(&conn, order_no).await?;
    Ok(status)
}

// 根据订单号查看获取订单详情
pub async fn get(order_no: &str) -> Result<Order> {
    let conn = Db::get_conn().await;

    let order = mtv_dao::order::get(&conn, order_no).await?;

    Ok(order)
}

// recharge_record_list 充值列表
pub async fn recharge_record_list(
    user_id: i32,
    page: i64,
    size: i64,
) -> Result<Page<Vec<RechargeRecord>>> {
    let conn = Db::get_conn().await;
    let page = mtv_dao::order::recharge_record_list(&conn, user_id, page, size).await?;
    Ok(page)
}

// consume_record_list
pub async fn consume_record_list(
    user_id: i32,
    page: i64,
    size: i64,
) -> Result<Page<Vec<ConsumeRecord>>> {
    let conn = Db::get_conn().await;
    let page = mtv_dao::order::consume_record_list(&conn, user_id, page, size).await?;
    Ok(page)
}

// 支付订单
pub async fn pay(order_no: &str, appid: &str, openid: &str) -> Result<impl Serialize> {
    let conn = Db::get_conn().await;

    // 获取订单信息
    let order = mtv_dao::order::get(&conn, order_no).await?;

    // 如果已经支付，直接返回订单编号
    // 订单状态 0:未支付 1:成功，-1失败
    if order.status == 1 {
        return Err("订单已支付，请重新创建订单".into());
    }

    let v = PAY
        .pay(
            appid,
            wxpay::PayType::JsApi,
            &order.description,
            &order.order_no,
            order.amount as u32,
            Some(openid.to_string()),
        )
        .await?;

    Ok(v)
}

pub async fn pay_notify(notify_data: WxPayNotify) -> Result<String> {
    let data = PAY.decode(&notify_data)?;

    let order_no = &data.out_trade_no;

    // 查询订单
    let conn = Db::get_conn().await;
    let order = mtv_dao::order::get(&conn, order_no).await?;

    // 如果已经支付，直接返回订单编号
    // 订单状态 0:未支付 1:成功，-1失败
    if order.status == 1 {
        return Ok(order.order_no);
    }

    let state = if notify_data.event_type == "TRANSACTION.SUCCESS" {
        1
    } else {
        -1
    };

    // 更新订单状态
    let order = mtv_dao::order::update_order_status(&conn, order_no.to_string(), state)
        .await
        .map_err(|e| {
            log::error!("更新订单状态出错: {}", e);
            "更新订单状态出错"
        })?;

    // 获取对应的商品
    let goods = mtv_dao::goods::get(&conn, order.goods_id).await?;
    if goods.is_none() {
        log::error!("商品不存在");
        return Err("商品不存在".into());
    }

    let goods = goods.unwrap();

    if goods.is_vip {
        let expire_time = match goods.expire_type {
            0 => chrono::Duration::days((30 * goods.expire_count).into()),
            1 => chrono::Duration::days((90 * goods.expire_count).into()),
            2 => chrono::Duration::days((365 * goods.expire_count).into()),
            _ => {
                log::error!("会员过期类型不支持");
                return Err("会员过期类型不支持".into());
            }
        };

        // 获取用户信息
        let user = mtv_dao::user::get(&conn, order.user_id).await?;
        let mut new_expire_time = user.vip_expire_time;
        let now = Local::now();
        // 如果vip到期时间大于当前时间，说明是续费
        if new_expire_time > now {
            // 续费，在到期的时间点添加
            new_expire_time = new_expire_time + expire_time;
        } else {
            // 开通,在当前基础添加
            new_expire_time = now + expire_time;
        }

        mtv_dao::user::update_vip(&conn, order.user_id, goods.expire_type, new_expire_time)
            .await
            .map_err(|e| {
                log::error!("更新用户会员出错: {}", e);
                "更新用户会员出错"
            })?;

        let description = format!(
            "开通会员{}{}",
            goods.expire_count,
            match goods.expire_type {
                0 => "个月",
                1 => "个季度",
                2 => "年",
                _ => {
                    log::error!("会员过期类型不支持");
                    return Err("会员过期类型不支持".into());
                }
            }
        );
        // 添加开会员充值记录
        mtv_dao::order::add_recharge_record(&conn, order.user_id, order.amount, 0, description)
            .await?;
    } else {
        mtv_dao::user::update_score(&conn, order.user_id, goods.score)
            .await
            .map_err(|e| {
                log::error!("更新用户积分出错: {}", e);
                "更新用户积分出错"
            })?;

        // 添加充值记录
        mtv_dao::order::add_recharge_record(
            &conn,
            order.user_id,
            order.amount,
            goods.score,
            goods.description,
        )
        .await?;
    }

    Ok(order.order_no)
}
