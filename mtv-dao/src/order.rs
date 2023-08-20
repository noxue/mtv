use chrono::Local;
use serde::Serialize;
use sqlrs::{Conn, Table};

use crate::Page;

#[derive(Debug, Clone, Serialize, Table)]
pub struct RechargeRecord {
    pub id: i32,
    pub user_id: i32,
    pub amount: i32,
    pub score: i32,
    pub mark: String,
    pub status: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

// 创建充值记录
pub async fn add_recharge_record(
    conn: &Conn,
    user_id: i32,
    amount: i32,
    score: i32,
    mark: String,
) -> anyhow::Result<u64> {
    let row = conn
        .execute(
            r#" insert into recharge_records (user_id, amount, score, mark) values ($1, $2, $3, $4)"#,
            &[&user_id, &amount, &score, &mark],
        )
        .await?;
    Ok(row)
}

// 查看指定用户的充值记录
pub async fn recharge_record_list(
    conn: &Conn,
    user_id: i32,
    page: i64,
    size: i64,
) -> anyhow::Result<Page<Vec<RechargeRecord>>> {
    let rows = conn
        .query(
            r#" select * from recharge_records where user_id = $1 order by create_time desc limit $2 offset $3 "#,
            &[&user_id, &size, &(&(page - 1) * size)],
        )
        .await?;
    let total = conn
        .query_one(
            r#" select count(*) from recharge_records where user_id = $1 "#,
            &[&user_id],
        )
        .await?;

    let total: i64 = total.get(0);

    let rrs = rows.iter().map(|row| row.try_into().unwrap()).collect();
    Ok(Page {
        total,
        page,
        size,
        data: rrs,
    })
}



#[derive(Debug, Clone, Serialize, Table)]
pub struct ConsumeRecord {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub video_id: i32,
    pub score: i32,
    pub mark: String,
    pub create_time: chrono::DateTime<Local>,
}

// 创建消费记录
pub async fn add_consume_record(
    conn: &Conn,
    user_id: i32,
    movie_id: i32,
    video_id: i32,
    score: i32,
    mark: String,
) -> anyhow::Result<ConsumeRecord> {
    let row = conn
        .query_one(
            r#" insert into consume_records (user_id, movie_id, video_id, score, mark) values ($1, $2, $3, $4, $5) returning * "#,
            &[&user_id, &movie_id, &video_id, &score, &mark],
        )
        .await?;
    Ok(row.try_into()?)
}

// 消费记录列表，分页
pub async fn consume_record_list(
    conn: &Conn,
    user_id: i32,
    page: i64,
    size: i64,
) -> anyhow::Result<Page<Vec<ConsumeRecord>>> {
    let rows = conn
        .query(
            r#" select * from consume_records where user_id = $1 order by create_time desc limit $2 offset $3 "#,
            &[&user_id, &size, &(&(page - 1) * size)],
        )
        .await?;
    let total = conn
        .query_one(
            r#" select count(*) from consume_records where user_id = $1 "#,
            &[&user_id],
        )
        .await?;

    let total: i64 = total.get(0);

    let crs = rows.iter().map(|row| row.try_into().unwrap()).collect();
    Ok(Page {
        total,
        page,
        size,
        data: crs,
    })
}

// 根据用户id和video_id 查询消费记录
pub async fn consume_record_by_user_id_and_video_id(
    conn: &Conn,
    user_id: i32,
    video_id: i32,
) -> anyhow::Result<bool> {
    let row = conn
        .query_opt(
            r#" select * from consume_records where user_id = $1 and video_id = $2 limit 1"#,
            &[&user_id, &video_id],
        )
        .await?;
    Ok(row.is_some())
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct Order {
    pub id: i32,
    pub goods_id: i32,
    pub user_id: i32,
    pub amount: i32,
    pub order_no: String,
    pub description: String,
    pub status: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

// 创建订单
pub async fn add(
    conn: &Conn,
    goods_id: i32,
    user_id: i32,
    amount: i32,
    order_no: String,
    description: String,
) -> anyhow::Result<Order> {
    let row = conn
        .query_one(
            r#" insert into orders (goods_id, user_id, amount, order_no, description) values ($1, $2, $3, $4, $5) returning * "#,
            &[&goods_id, &user_id, &amount, &order_no, &description],
        )
        .await?;
    Ok(row.try_into()?)
}

// 根据订单号查询订单
pub async fn get(conn: &Conn, order_no: &str) -> anyhow::Result<Order> {
    let row = conn
        .query_opt(
            r#" select * from orders where order_no = $1 limit 1 "#,
            &[&order_no],
        )
        .await?;
    if row.is_none() {
        // 订单不存在
        return Err(anyhow::anyhow!("订单不存在"));
    }
    Ok(row.unwrap().try_into()?)
}

// 根据订单号查询订单状态
pub async fn get_status(conn: &Conn, order_no: &str) -> anyhow::Result<i32> {
    let row = conn
        .query_opt(
            r#" select status from orders where order_no = $1 limit 1 "#,
            &[&order_no],
        )
        .await?;
    if row.is_none() {
        // 订单不存在
        return Err(anyhow::anyhow!("订单不存在"));
    }
    let row = row.unwrap();
    let status: i32 = row.get(0);
    Ok(status)
}

// 更新订单状态
pub async fn update_order_status(
    conn: &Conn,
    order_no: String,
    status: i32,
) -> anyhow::Result<Order> {
    let row = conn
        .query_one(
            r#" update orders set status = $1 where order_no = $2 returning * "#,
            &[&status, &order_no],
        )
        .await?;
    Ok(row.try_into()?)
}

// 根据用户id查询订单列表
pub async fn list_by_user_id(
    conn: &Conn,
    user_id: i32,
    page: i64,
    size: i64,
) -> anyhow::Result<Page<Vec<Order>>> {
    let rows = conn
        .query(
            r#" select * from orders where user_id = $1 order by create_time desc limit $2 offset $3 "#,
            &[&user_id, &size, &(&(page - 1) * size)],
        )
        .await?;
    let total = conn
        .query_one(
            r#" select count(*) from orders where user_id = $1 "#,
            &[&user_id],
        )
        .await?;

    let total: i64 = total.get(0);

    let orders = rows.iter().map(|row| row.try_into().unwrap()).collect();
    Ok(Page {
        total,
        page,
        size,
        data: orders,
    })
}
