use std::any;

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use mtv_config::CONFIG;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppId {
    pub appid: Option<String>,
}

impl AppId{
    
    pub fn get_appid(&self)->mtv_srv::Result<String>{
        match &self.appid {
            Some(appid) => Ok(appid.clone()),
            None => Err("header 中缺少 appid 参数".into()),
        }
    }
}

impl FromRequest for AppId {
    type Error = mtv_srv::SrvError;
    type Future = Ready<mtv_srv::Result<AppId>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        // http头获取token字段的值
        match req.headers().get("appid"){
            Some(appid) => {
                let appid = appid.to_str().unwrap();
                ok(AppId{appid:Some(appid.to_string())})
            },
            None => ok(AppId{appid:None})
        }
    }
}

