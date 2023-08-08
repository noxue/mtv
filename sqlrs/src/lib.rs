pub mod crud;

pub use sqlrs_macros::Table;
use std::sync::{Mutex, MutexGuard};
use tokio::sync::OnceCell;
use tokio_postgres::{Client, NoTls};

static DB: OnceCell<Mutex<Db>> = OnceCell::const_new();

pub struct Conn {
    conn: MutexGuard<'static, Db>,
}

impl std::ops::Deref for Conn {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl std::ops::DerefMut for Conn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

impl Conn {
    // begin
    pub async fn begin(&mut self) -> anyhow::Result<()> {
        self.conn.execute("begin", &[]).await?;
        Ok(())
    }

    // commit
    pub async fn commit(&mut self) -> anyhow::Result<()> {
        self.conn.execute("commit", &[]).await?;
        Ok(())
    }

    // rollback
    pub async fn rollback(&mut self) -> anyhow::Result<()> {
        self.conn.execute("rollback", &[]).await?;
        Ok(())
    }
}

/// 事务宏
/// 用法：
/// transaction! {
///    conn,{
///       conn.execute("insert into users (name) values ('admin')", &[]).await?;
///   }
/// }
#[macro_export]
macro_rules! transaction {
    ($conn:ident,{
        $($body:tt)*
    }) => {
        $conn.begin().await.unwrap();
        $($body)*
        $conn.commit().await.unwrap();
    };
}

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

    pub fn get_conn() -> Conn {
        let db = DB.get().expect("请先调用Db::init");
        let conn = db.lock().unwrap();
        Conn { conn }
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
