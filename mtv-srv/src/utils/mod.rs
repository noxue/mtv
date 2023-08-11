pub mod redis_client;

use lazy_static::lazy_static;

// lazy_static 定义全局的 redis Client
lazy_static! {
    pub static ref REDIS: redis::Client = redis_client::init();
}