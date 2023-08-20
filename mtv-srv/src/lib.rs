pub mod user;
pub mod utils;
pub mod movie;
pub mod order;
pub mod goods;

pub use utils::errors::{Result, SrvError};
pub use utils::REDIS;


