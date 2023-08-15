use actix_web::Responder;
use actix_web::Result;

use crate::middleware::Me;




// 充值记录列表
pub async fn recharge_list()->Result<impl Responder>{
    Ok("")
}

// 消费记录
pub async fn consume_list()->Result<impl Responder>{
    Ok("")
}

// 我的充值记录
pub async fn my_recharge_list(me:Me)->Result<impl Responder>{
    Ok("")
}

// 我的消费记录
pub async fn my_consume_list(me:Me)->Result<impl Responder>{
    Ok("")
}
