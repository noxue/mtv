use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlrs::{Conn, Table};

#[derive(Debug, Clone, Serialize, Table)]
pub struct Video {
    pub id: i32,
    pub movie_id: i32,
    pub name: String,
    pub video: String,
    pub price: i32,
    pub status: i32,
    pub likes: i32,
    pub share: i32,
    pub view: i32,
    pub rank: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

pub async fn add(
    conn: &Conn,
    movie_id: i32,
    name: String,
    video: String,
    price: i32,
    status: i32,
    rank: i32,
) -> anyhow::Result<Video> {
    let row = conn
        .query_one(
            r#" insert into videos (movie_id, name, video, price, status, rank) values ($1, $2, $3, $4, $5, $6) returning * "#,
            &[&movie_id, &name, &video, &price, &status, &rank],
        )
        .await?;
    Ok(row.try_into()?)
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct VideoList {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub rank: i32,
}

pub async fn list(conn: &Conn, movie_id: i32) -> anyhow::Result<Vec<VideoList>> {
    let rows = conn
        .query(
            r#" select id, name, price, rank from videos where movie_id = $1 order by rank asc, id asc"#,
            &[&movie_id],
        )
        .await?;
    Ok(rows.iter().map(|row| row.try_into().unwrap()).collect())
}

pub async fn get(conn: &Conn, video_id: i32) -> anyhow::Result<Option<Video>> {
    let row = conn
        .query_opt(r#" select * from videos where id = $1 "#, &[&video_id])
        .await?;

    if let Some(row) = row {
        Ok(Some(row.try_into()?))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateVideo {
    pub name: String,
    pub video: String,
    pub price: i32,
    pub status: i32,
    pub rank: i32,
}
pub async fn update(
    conn: &Conn,
    video_id: i32,
    update_video: &UpdateVideo,
) -> anyhow::Result<Video> {
    let update_time = chrono::Local::now();
    let row = conn
        .query_one(
            r#" update videos set name = $1, video = $2, price = $3, status = $4, rank = $5, update_time = $6 where id = $7 returning * "#,
            &[&update_video.name, &update_video.video, &update_video.price, &update_video.status, &update_video.rank, &update_time, &video_id],
        )
        .await?;
    Ok(row.try_into()?)
}

// 删除
pub async fn delete(conn: &Conn, video_id: i32) -> anyhow::Result<()> {
    let f = conn
        .execute(r#" delete from videos where id = $1 "#, &[&video_id])
        .await?;

    if f == 0 {
        return Err(anyhow::anyhow!("删除失败"));
    }

    Ok(())
}

// 修改点赞数 likes=likes+likes
pub async fn update_likes(conn: &Conn, video_id: i32, likes: i32) -> anyhow::Result<()> {
    let f = conn
        .execute(
            r#" update videos set likes = likes + $1 where id = $2 "#,
            &[&likes, &video_id],
        )
        .await?;

    if f == 0 {
        return Err(anyhow::anyhow!("修改点赞数失败"));
    }

    Ok(())
}

/*
CREATE TABLE likes_records (
    id serial PRIMARY KEY,
    user_id int4 NOT NULL,
    movie_id int4 NOT NULL,
    video_id int4 NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);
*/

// 点赞记录
#[derive(Debug, Clone, Serialize, Table)]
pub struct LikesRecord {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub video_id: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

pub async fn add_likes_record(
    conn: &Conn,
    user_id: i32,
    movie_id: i32,
    video_id: i32,
) -> anyhow::Result<()> {
    conn
        .query_one(
            r#" insert into likes_records (user_id, movie_id, video_id) values ($1, $2, $3) returning * "#,
            &[&user_id, &movie_id, &video_id],
        )
        .await?;
    Ok(())
}

// 删除
pub async fn delete_likes_record(conn: &Conn, user_id: i32, video_id: i32) -> anyhow::Result<()> {
    conn.execute(
        r#" delete from likes_records where user_id = $1 and video_id = $2 "#,
        &[&user_id, &video_id],
    )
    .await?;
    Ok(())
}

// 根据 user_id video_id 判断是否点赞
pub async fn is_liked(conn: &Conn, user_id: i32, video_id: i32) -> anyhow::Result<bool> {
    let row = conn
        .query_opt(
            r#" select id from likes_records where user_id = $1 and video_id = $2 limit 1 "#,
            &[&user_id, &video_id],
        )
        .await?;

    if let Some(_) = row {
        Ok(true)
    } else {
        Ok(false)
    }
}


#[derive(Debug, Clone, Serialize, Table)]
pub struct ViewRecord {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub movie_part_id: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct FollowRecord {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}
