use actix_web::Responder;
use actix_web::Result;

use crate::middleware::Me;


// 创建订单
pub async fn create()->Result<impl Responder>{
    Ok("")
}

// 订单列表
pub async fn list()->Result<impl Responder>{
    Ok("")
}

// 订单详情
pub async fn get()->Result<impl Responder>{
    Ok("")
}


// 充值记录列表
pub async fn recharges()->Result<impl Responder>{
    Ok("")
}

// 消费记录
pub async fn consumes()->Result<impl Responder>{
    Ok("")
}
