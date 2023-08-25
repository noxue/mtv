use crate::middleware::AppId;
use crate::utils::res::Res;
use actix_web::web;
use actix_web::Responder;
use actix_web::Result;
use mtv_srv::pay::WxPayNotify;
use serde::Deserialize;

// 查看订单支付情况
// 订单状态 0:未支付 1:成功，-1失败
pub async fn check(order_no: web::Path<String>) -> Result<impl Responder> {
    let order_no = order_no.into_inner();
    let r = mtv_srv::order::check(&order_no).await?;
    let mut res = Res::new();
    res.set_data(r);
    Ok(res)
}

#[derive(Debug, Deserialize, Clone)]
pub struct PayParam {
    pub order_no: String,
    pub openid: String,
}

// 生成支付签名
pub async fn pay(pay_param: web::Json<PayParam>, appid:AppId) -> Result<impl Responder> {

    let appid = appid.get_appid()?;

    let PayParam { order_no,  openid } = pay_param.into_inner();

    let r = mtv_srv::order::pay(&order_no, &appid, &openid).await?;
    let mut res = Res::new();
    res.set_data(r);

    Ok(res)
}

// 支付回调
pub async fn notify(notify_data: web::Json<WxPayNotify>) -> Result<impl Responder> {
    let data = mtv_srv::order::pay_notify(notify_data.into_inner()).await?;
    let mut res = Res::new();
    res.set_data(data);
    Ok(res)
}
