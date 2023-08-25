use std::any;

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use mtv_config::CONFIG;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Me {
    pub id: i32,
}

impl Me{
    pub fn is_admin(&self)->bool{
        CONFIG.admin_ids.contains(&self.id)
    }
    pub fn check_admin(&self)->mtv_srv::Result<()>{
        if !self.is_admin(){
            Err("您不是管理员".into())
        }else{
            Ok(())
        }
    }
}

impl FromRequest for Me {
    type Error = mtv_srv::SrvError;
    type Future = Ready<mtv_srv::Result<Me>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        // http头获取token字段的值
        let token = match req.headers().get("token") {
            Some(token) => token.to_str().unwrap(),
            None => return err("请先登录".into()),
        };
        match mtv_srv::user::get_uid(token.trim()).map(|id| Me { id }) {
            Ok(v) => ok(v),
            Err(e) => err(e.into()),
        }
    }
}

