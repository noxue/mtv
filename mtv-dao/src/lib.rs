pub mod user;

use crate::user::set_password;
use chrono::{Local, NaiveTime};
use postgres_types::FromSql;
use serde::{Deserialize, Serialize};
use serde_json::json;
pub use sqlrs::{transaction, Db, Table};

pub async fn test() {
    let mut db = Db::get_conn();
    // 开启事务

    transaction!(db, {
        set_password(&db, 1, "admin1").await.unwrap();
        let user = user::get(&db, 1).await.unwrap();
        // assert_eq!(user.auth.password, Some("admin1".to_string()));
    });

    // down().await;
    // up().await;
    // // for i in 1..10 {
    //     test_insert().await;
    // // }

    // // test_select().await;
    // test_find_one().await;
    // test_macro().await;
    // let db = Db::get_conn();

    // let rows = db
    //     .query("SELECT $1::TEXT", &[&format!("hello world")])
    //     .await
    //     .unwrap();
    // let value: &str = rows[0].get(0);
    // dbg!(value);
}

pub async fn test_insert() {
    let db = Db::get_conn();

    let auth = Auth {
        phone: None,
        password: None,
        wechat_unionid: Some("xxxxxxxxxxx".to_string()),
        wechat_openid: None,
    };

    let auth = json!(auth);

    let modified = db
        .execute(
            r#"
    insert into users (nickname, auth) values ($1, $2)
    "#,
            &[&"张三", &auth],
        )
        .await
        .unwrap();

    dbg!(modified);
}

// 根据表生成结构体User和Auth
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
    create_time: chrono::DateTime<Local>,
    update_time: chrono::DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    phone: Option<String>,
    password: Option<String>,
    wechat_unionid: Option<String>,
    wechat_openid: Option<String>,
}

pub async fn test_macro() {
    let fs = User::get_columns_vec();
    println!("get_columns:{:?}", fs);
    let fss = User::get_columns();
    println!("get_columns_str:{}", fss);
}

pub async fn test_find_one() {
    let db = Db::get_conn();

    let sql = format!("select {} from users where id = $1", User::get_columns());
    let row = db
        .query_one(&sql, &[&1])
        .await
        .unwrap_or_else(|_| panic!("未找到用户"));

    let user: User = row.try_into().unwrap();
    dbg!(user);
}

pub async fn test_select() {
    let db = Db::get_conn();

    let sql = format!(
        "select {} from users order by id asc limit 5",
        User::get_columns()
    );

    let rows = db.query(&sql, &[]).await.unwrap();

    let users: Vec<User> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    dbg!(users);
}

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

    let modified = db.batch_execute(include_str!("../down.sql")).await.unwrap();

    dbg!(modified);
}
