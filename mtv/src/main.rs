use mtv_dao::*;
use std::{error::Error, thread};
use sqlrs::Db;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Box<dyn Error>> {
    Db::init("host=localhost user=postgres password=123456 dbname=mtv")
        .await
        .unwrap();

    test().await;

    Ok(())
}
