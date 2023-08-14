use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use snafu::prelude::*;

pub type Result<T, E = SrvError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum SrvError {
    #[snafu(whatever, display("-1 = {message}"))]
    Error {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
    #[snafu(display("{code} = {msg}"))]
    Custom { code: i32, msg: String },
}

impl SrvError {
    pub fn new(code: i32, msg: &str) -> Self {
        SrvError::Custom {
            code,
            msg: msg.to_owned(),
        }
    }
    /// ### 支持以下几种错误信息类型
    /// 1. `错误码 = 错误信息`
    ///
    ///     `1000 = 用户名已存在` ， 100 就当做返回json中的 code，等号前后的空格会自动忽略，如果错误码不对， code设置为-100
    ///
    /// 1. `错误信息`
    ///
    ///     `内部错误` ， json中的code设置为-1，错误信息直接设置到 err中
    fn get_error(&self) -> (i32, String) {
        let err = self.to_string();
        match err.split_once("=") {
            Some(v) => match v.0.trim().parse() {
                Ok(v1) => (v1, v.1.trim_start().to_owned()),
                Err(_) => (-100, err),
            },
            None => (-1, err.trim_start().to_string()),
        }
    }
    pub fn code(&self) -> i32 {
        self.get_error().0
    }

    pub fn msg(&self) -> String {
        self.get_error().1
    }
}

impl ResponseError for SrvError {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse {
        
        HttpResponse::Ok().json(json!({
            "code": self.code(),
            "msg": self.msg()
        }))
    }
}

impl From<&'static str> for SrvError {
    fn from(e: &'static str) -> Self {
        SrvError::Error {
            message: e.to_string(),
            source: None,
        }
    }
}

// 从String转 SrvError
impl From<String> for SrvError {
    fn from(e: String) -> Self {
        SrvError::Error {
            message: e.to_string(),
            source: None,
        }
    }
}


// 从 anyhow::Error 转 SrvError
impl From<anyhow::Error> for SrvError {
    fn from(e: anyhow::Error) -> Self {
        SrvError::Error {
            message: e.to_string(),
            source: None,
        }
    }
}
