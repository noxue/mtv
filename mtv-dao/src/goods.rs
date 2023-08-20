use chrono::Local;
use serde::Serialize;
use sqlrs::{Conn, Table};

#[derive(Debug, Clone, Serialize, Table)]
pub struct Goods {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub description: String,
    pub score: i32,
    pub is_hot: bool,
    pub is_vip: bool,
    pub expire_type: i32,
    pub expire_count: i32,
    pub create_time: chrono::DateTime<Local>,
}

// 创建商品
pub async fn add(
    conn: &Conn,
    name: String,
    price: i32,
    description: String,
    score: i32,
    is_hot: bool,
    is_vip: bool,
    expire_type: i32,
    expire_count: i32,
) -> anyhow::Result<bool> {
    let row = conn
        .execute(
            r#" insert into goods (name, price, description, score, is_hot, is_vip, expire_type, expire_count) values ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            &[&name, &price, &description, &score, &is_hot, &is_vip, &expire_type, &expire_count],
        )
        .await?;
    Ok(row > 0)
}

// 商品列表
pub async fn list(conn: &Conn) -> anyhow::Result<Vec<Goods>> {
    let rows = conn.query(r#" select * from goods "#, &[]).await?;
    let goods = rows.iter().map(|row| row.try_into().unwrap()).collect();
    Ok(goods)
}

// 根据商品id查询商品
pub async fn get(conn: &Conn, id: i32) -> anyhow::Result<Option<Goods>> {
    let row = conn
        .query_opt(r#" select * from goods where id = $1 limit 1 "#, &[&id])
        .await?;
    if row.is_none() {
        return Err(anyhow::anyhow!("商品不存在"));
    }
    Ok(Some(row.unwrap().try_into()?))
}

// 删除商品
pub async fn delete(conn: &Conn, id: i32) -> anyhow::Result<()> {
    conn.execute(r#" delete from goods where id = $1 "#, &[&id])
        .await?;
    Ok(())
}

// 更新商品
pub async fn update(
    conn: &Conn,
    id: i32,
    name: String,
    price: i32,
    description: String,
    score: i32,
    is_hot: bool,
    is_vip: bool,
    expire_type: i32,
    expire_count: i32,
) -> anyhow::Result<bool> {
    let row = conn
        .execute(
            r#" update goods set name = $1, price = $2, description = $3, score = $4, is_hot = $5, is_vip = $6, expire_type = $7, expire_count = $8 where id = $9 returning * "#,
            &[&name, &price, &description, &score, &is_hot, &is_vip, &expire_type, &expire_count, &id],
        )
        .await?;
    if row == 0 {
        return Err(anyhow::anyhow!("商品不存在"));
    }
    Ok(true)
}
