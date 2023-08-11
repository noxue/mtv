use std::env;

use mtv_config::CONFIG;
use redis::Client;

pub fn init() -> Client {
    redis::Client::open(CONFIG.redis_url.as_str()).expect("无法连接到 Redis")
}
