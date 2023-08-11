mod api;
mod utils;

use actix_web::{App, HttpServer};
use mtv_config::CONFIG;
use sqlrs::Db;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    fast_log::init(
        fast_log::Config::new()
            .level(CONFIG.log_level.parse().unwrap())
            .console()
            .file_split(
                "logs/",
                fast_log::consts::LogSize::MB(1),
                fast_log::plugin::file_split::RollingType::All,
                fast_log::plugin::packer::LogPacker {},
            ),
    )
    .unwrap();

    Db::init(&CONFIG.db).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // 跨域,允许所有请求
            .wrap(actix_cors::Cors::permissive())
            .service(crate::api::api())
    })
    .bind(&CONFIG.app_host)?
    .run()
    .await?;

    Ok(())
}
