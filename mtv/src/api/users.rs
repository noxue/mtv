use actix_web::{web, HttpResponse, Responder};
use anyhow::Ok;
use serde::Deserialize;

use mtv_srv as srv;

use crate::utils::res::Res;

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub code: String,
    pub login_type: String, // mp or  weapp
}

pub async fn login(data: web::Json<LoginInfo>) ->  actix_web::Result<impl Responder> {
    println!("login: {:?}", data);
    let LoginInfo { code, login_type } = data.into_inner();

    let res = srv::users::login(&code, &login_type).await;
    log::debug!("login res: {:?}", res);


   Ok("t")
}
