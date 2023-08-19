use actix_web::web;
use actix_web::Responder;
use actix_web::Result;
use mtv_srv::utils::pay;
use mtv_srv::utils::pay::WxPayNotify;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::res::Res;

// 查看订单支付情况
pub async fn check() -> Result<impl Responder> {
    Ok("")
}

#[derive(Debug, Deserialize, Clone)]
pub struct PayParam {
    pub order_no: String,
    pub openid: String,
}

// 生成支付签名
pub async fn pay(pay_param: web::Json<PayParam>) -> Result<impl Responder> {
    let PayParam { order_no, openid } = pay_param.into_inner();

    let r = mtv_srv::order::pay(&order_no, &openid).await?;
    let mut res = Res::new();
    res.set_data(r);

    Ok(res)
}

// 支付回调
pub async fn notify(
    notify_data: web::Json<WxPayNotify>,
) -> Result<impl Responder> {

    Ok("")
}
