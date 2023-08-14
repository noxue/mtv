use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use mtv_srv as srv;

use crate::{middleware::Me, utils::res::Res};

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub code: String,
    pub login_type: String, // mp or  weapp
}

pub async fn login(data: web::Json<LoginInfo>) -> actix_web::Result<impl Responder> {
    println!("login: {:?}", data);
    let LoginInfo { code, login_type } = data.into_inner();

    let token = srv::users::login(&code, &login_type).await?;
    log::debug!("login res: {:?}", token);

    let mut res = Res::new();
    res.set_data(token);

    Ok(res)
}

pub async fn me(me: Me) -> actix_web::Result<impl Responder> {
    let user = mtv_srv::users::get(me.id).await?;

    let mut res = Res::new();
    res.set_data(user);
    Ok(res)
}


// 分页列出用户

