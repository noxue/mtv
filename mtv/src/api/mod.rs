pub mod movies;
pub mod pay;
pub mod scores;
pub mod users;

use actix_web::{web, Scope};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    page: Option<i64>,
    size: Option<i64>,
}

async fn down_up() -> &'static str {
    mtv_srv::users::down_up().await.unwrap();
    "xx"
}

pub fn api() -> Scope {
    web::scope("/api")
        .route("/down_up", web::get().to(down_up))
        .service(
            web::scope("/users")
                .route("", web::get().to(users::users))
                .route("/channel/{channel}", web::get().to(users::users_by_channel))
                .route("/login", web::post().to(users::login))
                .route("/me", web::get().to(users::me)),
        )
}
