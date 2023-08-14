use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlrs::{Conn, Db, Table};

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

/// 根据userid 设置用户密码
pub async fn set_password(conn: &Conn, userid: i32, password: &str) -> anyhow::Result<()> {
    // let row = db
    //     .execute(
    //         r#"
    // update users set auth = jsonb_set(auth, '{password}', $1::jsonb) where id = $2
    // "#,
    //         &[&password, &userid],
    //     )
    //     .await?;

    // 上面代码报错  panicked at 'called `Result::unwrap()` on an `Err` value: error serializing parameter 0: cannot convert between the Rust type `&str` and the Postgres type `jsonb`
    // 请修改
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

    // panic!("xxxxxxxxxxxxxxxxxxxxxxx");

    Ok(())
}

/// 设置手机和密码
pub async fn set_phone_and_password(
    conn: &Conn,
    userid: i32,
    phone: &str,
    password: &str,
) -> anyhow::Result<()> {
    let row = conn
        .execute(
            r#"
    update users set auth = jsonb_set(auth, '{phone}', $1::jsonb, '{password}', $2::jsonb) where id = $3
    "#,
            &[&json!(phone), &json!(password), &userid],
        )
        .await?;

    if row == 0 {
        anyhow::bail!("未找到用户");
    }

    Ok(())
}

/// 根据id获取用户
pub async fn get(conn: &Conn, userid: i32) -> anyhow::Result<User> {
    let row = conn
        .query_one(
            r#"
    select * from users where id = $1
    "#,
            &[&userid],
        )
        .await?;

    let user: User = row.try_into()?;
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
            r#" select * from users where auth->>'wechat_unionid' = $1 or auth->>'wechat_openid' = $2 "#,
            &[&unionid, &openid],
        )
        .await?;

    if let Some(row) = row {
        let user: User = row.try_into()?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
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
        anyhow::bail!("积分不足");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn a() {}

    #[tokio::test]
    async fn test_set_user_password_by_userid() {
        Db::init("host=localhost user=postgres password=123456 dbname=mtv")
            .await
            .unwrap();

        let mut conn = Db::get_conn();
        // 开启事务

        conn.begin().await.unwrap();

        set_password(&conn, 1, "admin10").await.unwrap();
        conn.rollback().await.unwrap();

        let user = get(&conn, 1).await.unwrap();
        assert_eq!(user.auth.password, Some("admin9".to_string()));

        // 根据unionid获取用户
        // let user = get_user_by_unionid(unionid).await.unwrap();
        // assert_eq!(user.auth.wechat_unionid, Some(unionid.to_string()));
    }

    #[tokio::test]
    async fn test_create_user_by_unionid() {
        Db::init("host=localhost user=postgres password=123456 dbname=mtv")
            .await
            .unwrap();

        let conn = Db::get_conn();
        let unionid = "bbbbbbbbbbbbbbbbbbbbbbb";
        let user = create_user_by_unionid_and_openid(&conn, unionid, "openid")
            .await
            .unwrap();
        assert_eq!(user.auth.wechat_unionid, Some(unionid.to_string()));
        // 删除
        delete(&conn, user.id).await.unwrap();
    }

    // update_score
    #[tokio::test]
    async fn test_update_score() {
        Db::init("host=localhost user=postgres password=123456 dbname=mtv")
            .await
            .unwrap();

        let conn = Db::get_conn();
        let userid = 1;
        let score = -100;
        update_score(&conn, userid, score).await.unwrap();
    }
}
