use mtv_dao::*;
use sqlrs::Db;
use std::error::Error;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Box<dyn Error>> {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    Db::init("host=localhost user=postgres password=123456 dbname=mtv")
        .await
        .unwrap();

    test().await;

    Ok(())
}
