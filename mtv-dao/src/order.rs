
use chrono::Local;
use serde::Serialize;
use sqlrs::Table;


#[derive(Debug, Clone, Serialize, Table)]
pub struct RechargeRecord{
    pub id: i32,
    pub user_id: i32,
    pub amount: i32,
    pub score: i32,
    pub mark: String,
    pub status: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}


#[derive(Debug, Clone, Serialize, Table)]
pub struct ConsumeRecord{
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub movie_part_id: i32,
    pub score: i32,
    pub mark: String,
    pub create_time: chrono::DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct Order{
    pub id: i32,
    pub user_id: i32,
    pub amount: i32,
    pub order_no: String,
    pub status: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}



