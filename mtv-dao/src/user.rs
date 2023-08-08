use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlrs::{Db, Table};

#[derive(Debug, Table)]
pub struct User {
    id: i32,
    nickname: Option<String>,
    avatar: Option<String>,
    score: i32,
    vip: i32,
    vip_expire_time: chrono::DateTime<Local>,
    #[sql_json]
    auth: Auth,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    phone: Option<String>,
    password: Option<String>,
    wechat_unionid: Option<String>,
    wechat_openid: Option<String>,
}

/// 根据unionid创建用户
pub async fn create_user_by_unionid(unionid: &str) -> anyhow::Result<User> {
    let db = Db::get_conn();

    let auth = Auth {
        phone: None,
        password: None,
        wechat_unionid: Some(unionid.to_string()),
        wechat_openid: None,
    };

    let auth = json!(auth);

    let row = db
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
pub async fn update(
    userid: i32,
    nickname: &str,
    avatar: &str,
) -> anyhow::Result<()> {
    let mut db = Db::get_conn();
    let row = db
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
pub async fn set_password(userid: i32, password: &str) -> anyhow::Result<()> {
    let mut db = Db::get_conn();
    let db = db.transaction().await?;

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
    let row = db
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
    db.commit().await.unwrap();

    // panic!("xxxxxxxxxxxxxxxxxxxxxxx");
    
    Ok(())
}

/// 设置手机和密码
pub async fn set_phone_and_password(
    userid: i32,
    phone: &str,
    password: &str,
) -> anyhow::Result<()> {
    let db = Db::get_conn();

    let row = db
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
pub async fn get(userid: i32) -> anyhow::Result<User> {
    let db = Db::get_conn();

    let row = db
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
pub async fn get_by_unionid(unionid: &str) -> anyhow::Result<User> {
    let db = Db::get_conn();

    let row = db
        .query_one(
            r#"
    select * from users where auth->>'wechat_unionid' = $1
    "#,
            &[&unionid],
        )
        .await?;

    let user: User = row.try_into()?;
    Ok(user)
}

/// 根据id删除用户
pub async fn delete(userid: i32) -> anyhow::Result<()> {
    let db = Db::get_conn();

    let row = db
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

        set_password(1, "admin6").await.unwrap();
        // asert password
        let user = get(1).await.unwrap();
        assert_eq!(user.auth.password, Some("admin6".to_string()));

        // 根据unionid获取用户
        // let user = get_user_by_unionid(unionid).await.unwrap();
        // assert_eq!(user.auth.wechat_unionid, Some(unionid.to_string()));

    }

    #[tokio::test]
    async fn test_create_user_by_unionid(){
        Db::init("host=localhost user=postgres password=123456 dbname=mtv")
        .await
        .unwrap();

        let unionid = "bbbbbbbbbbbbbbbbbbbbbbb";
        let user = create_user_by_unionid(unionid).await.unwrap();
        assert_eq!(user.auth.wechat_unionid, Some(unionid.to_string()));
        // 删除
        delete(user.id).await.unwrap();

    }



}
