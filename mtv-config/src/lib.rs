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

    pub oss_api_host: String,
    pub oss_access_key_id: String,
    pub oss_access_key_secret: String,
    pub oss_host: String,
    pub oss_callback_url: String,
    pub oss_bucket_name: String,
    pub oss_expire_time: i32,

    pub wx_mp_app_id: String,
    pub wx_mp_app_secret: String,
    pub wx_pay_api_host: String,
    pub wx_pay_mch_id: String,
    pub wx_pay_api_v3_private_key: String,
    pub wx_pay_serial_no: String,
    pub wx_pay_notify_url: String,
    pub wx_pay_cert_path: String,
    pub wx_pay_cert_key_path: String,
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

        let wx_mp_app_id = env::var("WX_MP_APP_ID").expect("WX_MP_APP_ID must be set");
        let wx_mp_app_secret = env::var("WX_MP_APP_SECRET").expect("WX_MP_APP_SECRET must be set");
        let wx_pay_api_host = env::var("WX_PAY_API_HOST").expect("WX_PAY_API_HOST must be set");
        let wx_pay_mch_id = env::var("WX_PAY_MCH_ID").expect("WX_PAY_MCH_ID must be set");
        let wx_pay_api_v3_private_key = env::var("WX_PAY_API_V3_PRIVATE_KEY").expect("WX_PAY_API_V3_PRIVATE_KEY must be set");
        let wx_pay_serial_no = env::var("WX_PAY_SERIAL_NO").expect("WX_PAY_SERIAL_NO must be set");
        let wx_pay_notify_url = env::var("WX_PAY_NOTIFY_URL").expect("WX_PAY_NOTIFY_URL must be set");
        let wx_pay_cert_path = env::var("WX_PAY_CERT_PATH").expect("WX_PAY_CERT_PATH must be set");
        let wx_pay_cert_key_path = env::var("WX_PAY_CERT_KEY_PATH").expect("WX_PAY_CERT_KEY_PATH must be set");

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
            wx_mp_app_id,
            wx_mp_app_secret,
            wx_pay_api_host,
            wx_pay_mch_id,
            wx_pay_api_v3_private_key,
            wx_pay_serial_no,
            wx_pay_notify_url,
            wx_pay_cert_path,
            wx_pay_cert_key_path,
        }
    }
}
