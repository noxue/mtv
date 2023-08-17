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
    pub admin_ids: Vec<i32>,
    /*
    # oss本地服务接口
OSS_API_HOST = http://127.0.0.1:3000
OSS_ACCESS_KEY_ID = LTAI4FjJ8Qq4X3Z4
OSS_ACCESS_KEY_SECRET = 11111111111
OSS_HOST = http://mtv.oss-cn-beijing.aliyuncs.com
OSS_CALLBACK_URL = http://xxxxxxxxxx.com
OSS_BUCKET_NAME = mtv
# token超时时间
OSS_EXPIRE_TIME = 30
     */
    pub oss_api_host: String,
    pub oss_access_key_id: String,
    pub oss_access_key_secret: String,
    pub oss_host: String,
    pub oss_callback_url: String,
    pub oss_bucket_name: String,
    pub oss_expire_time: i32,
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
        let admin_ids = env::var("ADMIN_IDS")
            .unwrap_or_default()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let oss_api_host = env::var("OSS_API_HOST").expect("OSS_API_HOST must be set");
        let oss_access_key_id = env::var("OSS_ACCESS_KEY_ID").expect("OSS_ACCESS_KEY_ID must be set");
        let oss_access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").expect("OSS_ACCESS_KEY_SECRET must be set");
        let oss_host = env::var("OSS_HOST").expect("OSS_HOST must be set");
        let oss_callback_url = env::var("OSS_CALLBACK_URL").expect("OSS_CALLBACK_URL must be set");
        let oss_bucket_name = env::var("OSS_BUCKET_NAME").expect("OSS_BUCKET_NAME must be set");
        let oss_expire_time = env::var("OSS_EXPIRE_TIME").expect("OSS_EXPIRE_TIME must be set").parse::<i32>().unwrap();


        DataConfig {
            log_level,
            db,
            app_host,
            weapp_appid,
            weapp_secret,
            redis_url,
            admin_ids,
            oss_api_host,
            oss_access_key_id,
            oss_access_key_secret,
            oss_host,
            oss_callback_url,
            oss_bucket_name,
            oss_expire_time,
        }
    }
}
