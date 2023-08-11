use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: DataConfig = DataConfig::init();
}

pub struct DataConfig {
    pub log_level: String,
    pub db: String,
    pub app_host: String,
    pub weapp_appid: String,
    pub weapp_secret: String,
    pub redis_url: String,
}

impl DataConfig {
    fn init() -> DataConfig {
        dotenv::dotenv().ok();
        let log_level = env::var("LOG_LEVEL").unwrap_or("WARN".to_string());
        let db = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
        let weapp_appid = env::var("WEAPP_APPID").expect("WEAPP_APPID must be set");
        let weapp_secret = env::var("WEAPP_SECRET").expect("WEAPP_SECRET must be set");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");


        DataConfig {
            log_level,
            db,
            app_host,
            weapp_appid,
            weapp_secret,
            redis_url,
        }
    }
}
