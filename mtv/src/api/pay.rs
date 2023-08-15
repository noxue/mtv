use actix_web::Responder;
use actix_web::Result;

// 创建支付订单
pub async fn create_order() -> Result<impl Responder> {
    Ok("")
}

// 查看订单支付情况
pub async fn check_order() -> Result<impl Responder> {
    Ok("")
}

// 生成支付签名
pub async fn sign() -> Result<impl Responder> {
    Ok("")
}

// 支付回调
pub async fn callback() -> Result<impl Responder> {
    Ok("")
}
