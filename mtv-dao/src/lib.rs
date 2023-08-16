pub mod user;
pub mod movie;
pub mod order;

use serde::Serialize;
pub use sqlrs::{transaction, Db, Table};

pub async fn up() {
    let db = Db::get_conn();

    let modified = db
        // init.sql
        .batch_execute(include_str!("../up.sql"))
        .await
        .unwrap();

    dbg!(modified);
}

pub async fn down() {
    let db = Db::get_conn();

    let modified = db.batch_execute(include_str!("../down.sql")).await;

    dbg!(modified);
}


// 分页结果
#[derive(Debug, Serialize)]
pub struct Page<T>
where
    T: Serialize,
{
    pub page: i64,
    pub size: i64,
    pub total: i64,
    pub data: T,
}
