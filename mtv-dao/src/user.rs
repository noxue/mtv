use anyhow::Context;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlrs::{Conn, Table};

use crate::Page;

#[derive(Debug, Table, Serialize)]
pub struct User {
    pub id: i32,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub score: i32,
    pub vip: i32,
    pub vip_expire_time: chrono::DateTime<Local>,
    #[sql_json]
    pub auth: Auth,
    pub channel: Option<String>,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub phone: Option<String>,
    pub password: Option<String>,
    pub wechat_unionid: Option<String>,
    pub wechat_openid: Option<String>,
}

/// 根据unionid创建用户
pub async fn create_user_by_unionid_and_openid(
    conn: &Conn,
    unionid: &str,
    openid: &str,
) -> anyhow::Result<User> {
    let auth = Auth {
        phone: None,
        password: None,
        wechat_unionid: Some(unionid.to_string()),
        wechat_openid: Some(openid.to_string()),
    };

    let auth = json!(auth);

    let row = conn
        .query_one(
            r#"
    insert into users (auth) values ($1) returning *
    "#,
            &[&auth],
        )
        .await?;

    let user: User = row.try_into()?;
    Ok(user)
}

/// 根据userid 更新信息
pub async fn update(conn: &Conn, userid: i32, nickname: &str, avatar: &str) -> anyhow::Result<()> {
    let row = conn
        .execute(
            r#"
    update users set nickname = $1, avatar = $2 where id = $3
    "#,
            &[&nickname, &avatar, &userid],
        )
        .await?;

    if row == 0 {
        anyhow::bail!("未找到用户");
    }

    Ok(())
}

pub async fn set_channel(conn: &Conn, userid: i32, channel: &str) -> anyhow::Result<()> {
    let row = conn
        .execute(
            r#"
    update users set channel = $1 where id = $2
    "#,
            &[&channel, &userid],
        )
        .await?;

    if row == 0 {
        anyhow::bail!("未找到用户");
    }

    Ok(())
}

// 根据渠道列出用户,分页
pub async fn list_by_channel(
    conn: &Conn,
    channel: &str,
    page: i64,
    size: i64,
) -> anyhow::Result<Page<Vec<User>>> {
    let rows = conn
        .query_one(
            r#"
    select count(*) from users where channel = $1
    "#,
            &[&channel],
        )
        .await?;

    let total: i64 = rows.get(0);

    let rows = conn
        .query(
            r#"
    select * from users where channel = $1 order by id desc limit $2 offset $3
    "#,
            &[&channel, &size, &(&(page - 1) * size)],
        )
        .await?;

    let users: Vec<User> = rows
        .iter()
        .map(|row| row.try_into().context("row转换到用户信息出错").unwrap())
        .collect();

    Ok(Page {
        total,
        page,
        size,
        data: users,
    })
}

// 列出所有用户，分页
pub async fn list(conn: &Conn, page: i64, size: i64) -> anyhow::Result<Page<Vec<User>>> {
    let rows = conn
        .query_one(
            r#"
    select count(*) from users
    "#,
            &[],
        )
        .await?;

    let total = rows.get(0);

    let rows = conn
        .query(
            r#"
    select * from users order by id desc limit $1 offset $2
    "#,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;

    let users: Vec<User> = rows
        .iter()
        .map(|row| row.try_into().context("row转换到用户信息出错").unwrap())
        .collect();

    Ok(Page {
        total,
        page,
        size,
        data: users,
    })
}

/// 根据userid 设置用户密码
pub async fn set_password(conn: &Conn, userid: i32, password: &str) -> anyhow::Result<()> {
    let row = conn
        .execute(
            r#"
    update users set auth = jsonb_set(auth, '{password}', $1::jsonb) where id = $2
    "#,
            &[&json!(password), &userid],
        )
        .await?;
    if row == 0 {
        anyhow::bail!("未找到用户");
    }

    Ok(())
}

/// 设置手机和密码
pub async fn set_phone_and_password(
    conn: &Conn,
    userid: i32,
    phone: &str,
    password: &str,
) -> anyhow::Result<()> {
    let m = conn
        .execute(
            r#"
    update users set auth = jsonb_set(auth, '{phone}', $1::jsonb) where id = $2
    "#,
            &[&json!(phone), &userid],
        )
        .await?;

    // set password
    let n = conn
        .execute(
            r#"
    update users set auth = jsonb_set(auth, '{password}', $1::jsonb) where id = $2
    "#,
            &[&json!(password), &userid],
        )
        .await?;
    if m + n == 0 {
        anyhow::bail!("未找到用户");
    }

    Ok(())
}

/// 根据id获取用户
pub async fn get(conn: &Conn, userid: i32) -> anyhow::Result<User> {
    let row = conn
        .query_opt(
            r#"
    select * from users where id = $1
    "#,
            &[&userid],
        )
        .await?;

    if row.is_none() {
        anyhow::bail!("未找到用户");
    }

    let user: User = row.unwrap().try_into()?;
    Ok(user)
}

/// 根据union获取用户
pub async fn get_by_unionid_or_openid(
    conn: &Conn,
    unionid: &str,
    openid: &str,
) -> anyhow::Result<Option<User>> {
    let row = conn
        .query_opt(
            r#"select * from users where auth->>'wechat_openid' = $1 or (auth->>'wechat_unionid' = $2 and auth->>'wechat_unionid' <> '')"#,
            &[ &openid, &unionid],
        )
        .await?;

    if let Some(row) = row {
        let user: User = row.try_into()?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

// 根据手机查找用户
pub async fn get_by_phone(conn: &Conn, phone: &str) -> anyhow::Result<User> {
    let row = conn
        .query_opt(
            r#"
    select * from users where auth->>'phone' = $1
    "#,
            &[&phone],
        )
        .await?;

    if row.is_none() {
        anyhow::bail!("未找到用户");
    }

    Ok(row.unwrap().try_into()?)
}

/// 根据id删除用户
pub async fn delete(conn: &Conn, userid: i32) -> anyhow::Result<()> {
    let row = conn
        .execute(
            r#"
    delete from users where id = $1
    "#,
            &[&userid],
        )
        .await?;

    if row == 0 {
        anyhow::bail!("未找到用户");
    }

    Ok(())
}

/// 增加减少积分，保证积分不会减少到负数
pub async fn update_score(conn: &Conn, userid: i32, score: i32) -> anyhow::Result<()> {
    let row = conn
        .execute(
            r#" update users set score = score + $1 where id = $2 and score + $1 >= 0"#,
            &[&score, &userid],
        )
        .await?;

    if row == 0 {
        anyhow::bail!("金币不足,请充值");
    }
    Ok(())
}

pub async fn update_vip(
    conn: &Conn,
    userid: i32,
    expire_type: i32,
    expire_time: chrono::DateTime<Local>,
) -> anyhow::Result<()> {
    let row = conn
        .execute(
            r#"
    update users set vip = $1, vip_expire_time = $2 where id = $3
    "#,
            &[&expire_type, &expire_time, &userid],
        )
        .await?;

    if row == 0 {
        anyhow::bail!("未找到用户");
    }

    Ok(())
}
