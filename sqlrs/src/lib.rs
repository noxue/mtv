pub mod crud;

pub use sqlrs_macros::Table;
use std::sync::{Mutex, MutexGuard};
use tokio::sync::OnceCell;
use tokio_postgres::{Client, NoTls};

static DB: OnceCell<Mutex<Db>> = OnceCell::const_new();

#[derive(Debug)]
pub struct Db {
    client: Client,
}

impl Db {
    pub async fn init(conn_str: &str) -> anyhow::Result<()> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        DB.set(Mutex::new(Db { client }))
            .expect("初始化函数只能调用一次");
        Ok(())
    }

    pub fn get_conn() -> MutexGuard<'static, Db> {
        DB.get().expect("请先调用初始化函数").lock().unwrap()
    }
}

impl std::ops::Deref for Db {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl std::ops::DerefMut for Db {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
