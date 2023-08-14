pub mod users;
pub mod movies;

use actix_web::{web, Scope};



async fn hello() -> &'static str{
    "Hello world!"
}

pub fn api() -> Scope {
    web::scope("/api")
    .service(
        web::scope("/users")
        .route("/login", web::post().to(users::login))
        .route("/me", web::get().to(users::me))
        .route("/test", web::get().to(hello))
    )
}
