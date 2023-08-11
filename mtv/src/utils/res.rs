use actix_web::{body::EitherBody, error::JsonPayloadError, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use std::fmt::{Debug, Display};

#[derive(Debug, Serialize)]
pub struct Res<T = String>
where
    T: Serialize,
{
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T> Display for Res<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code:{},err:{}", self.code, self.msg)
    }
}

impl<T> Res<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self {
            code: 0,
            msg: "".to_owned(),
            data: None,
        }
    }
}

impl<T> Res<T>
where
    T: Serialize,
{
    pub fn set_data(&mut self, data: T) -> &Self {
        self.data = Some(data);
        self
    }

    pub fn set_err(&mut self, err: &str) -> &mut Self {
        self.code = -1;
        self.msg = err.to_owned();
        self
    }
}

impl<T: Serialize> Responder for Res<T> {
    type Body = EitherBody<String>;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(body) => match HttpResponse::Ok()
                .content_type("application/json")
                .message_body(body)
            {
                Ok(res) => res.map_into_left_body(),
                Err(err) => HttpResponse::from_error(err).map_into_right_body(),
            },

            Err(err) => {
                HttpResponse::from_error(JsonPayloadError::Serialize(err)).map_into_right_body()
            }
        }
    }
}



impl<T> From<anyhow::Result<T>> for Res<T>
where
    T: Serialize,
{
    fn from(res: anyhow::Result<T>) -> Self {
        match res {
            Ok(data) => {
                let mut res = Self::new();
                res.set_data(data);
                res
            }
            Err(err) => {
                let mut res = Self::new();
                res.set_err(&err.to_string());
                res
            }
        }
    }
}

// 分页结果
#[derive(Debug, Serialize)]
pub struct Page<T>
where
    T: Serialize,
{
    pub page: usize,
    pub size: usize,
    pub total: usize,
    pub data: T,
}
