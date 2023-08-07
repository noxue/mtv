extern crate sqlrs_macros;

use chrono::{Local, NaiveTime};
use postgres_types::FromSql;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlrs::Db;
use sqlrs_macros::Table;
use std::time::SystemTime;

pub async fn test() {
    // down().await;
    // up().await;
    // test_insert().await;
    test_select().await;
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

    let info = UserInfo {
        name: "张三".to_string(),
        password: "123456".to_string(),
    };

    let info = json!(info);

    let modified = db
        .execute(
            r#"
    insert into users (name, age, info) values ($1, $2, $3)
    "#,
            &[&"张三", &7, &info],
        )
        .await
        .unwrap();

    dbg!(modified);
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct UserInfo {
    #[serde(rename = "name")]
    name: String,
    password: String,
}

#[derive(Debug, Table)]
pub struct User {
    id: i32,
    name: String,
    age: i32,
    #[sql_json]
    info: UserInfo,
    created_at: chrono::DateTime<Local>,
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
    let user: User = db
        .query_one(&sql, &[&1i32])
        .await
        .unwrap()
        .try_into()
        .unwrap();

    dbg!(user);
}

pub async fn test_select() {
    let db = Db::get_conn();

    let sql = format!("select {} from users", User::get_columns());

    let rows = db.query(&sql, &[]).await.unwrap();

    let users: Vec<User> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    dbg!(users);
}

pub async fn up() {
    let db = Db::get_conn();

    let modified = db
        .execute(
            r#"
    create table if not exists users (
        id serial primary key,
        name varchar not null,
        info json not null,
        age int not null,
        created_at timestamp with time zone not null default now()
    )
    "#,
            &[],
        )
        .await
        .unwrap();

    dbg!(modified);
}

pub async fn down() {
    let db = Db::get_conn();

    let modified = db
        .execute(
            r#"
    drop table if exists users
    "#,
            &[],
        )
        .await
        .unwrap();

    dbg!(modified);
}
