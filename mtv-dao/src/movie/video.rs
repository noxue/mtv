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
    pub video_id: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

// 添加观看记录
pub async fn add_view_record(
    conn: &Conn,
    user_id: i32,
    movie_id: i32,
    video_id: i32,
) -> anyhow::Result<()> {
    conn.execute(
        r#" insert into view_records (user_id, movie_id, video_id) values ($1, $2, $3)"#,
        &[&user_id, &movie_id, &video_id],
    )
    .await?;
    Ok(())
}


#[derive(Debug, Clone, Serialize, Table)]
pub struct MovieHistory{
    pub id: i32,
    pub movie_name: String,
    pub cover: String,
    pub video_id: i32,
    pub video_name: String,
    pub create_time: chrono::DateTime<Local>,
}

pub async fn recent_view(
    conn: &Conn,
    user_id: i32,
) -> anyhow::Result<Vec<MovieHistory>> {
    // 列出观看历史，从 view_records 查询 创建时间， 然后根据 movie_id 分组, 查出最后插入的 video_id，和 create_time， 再从 movies 查出 name 作为 movie_name, cover, 从 videos 查出 name 作为 video_name  最后根据 view_records中create_time 倒序排列，并分页
    let rows = conn
        .query(
            r#" select m.id, m.name as movie_name, m.cover, v.id as video_id, v.name as video_name,max(r.create_time) as create_time from view_records as r left join movies as m on r.movie_id = m.id left join videos as v on r.video_id = v.id where r.user_id = $1 group by m.id, m.name, m.cover, v.id, v.name order by max(r.create_time) desc"#,
            &[&user_id],
        )
        .await?;
        
    Ok(rows.iter().map(|row| row.try_into().unwrap()).collect())
}

#[derive(Debug, Clone, Serialize, Table)]
pub struct FollowRecord {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}


// 添加追剧记录
pub async fn add_follow_record(
    conn: &Conn,
    user_id: i32,
    movie_id: i32,
) -> anyhow::Result<()> {
    conn.execute(
        r#" insert into follow_records (user_id, movie_id) values ($1, $2)"#,
        &[&user_id, &movie_id],
    )
    .await?;
    Ok(())
}

// 根据 user_id movie_id 判断是否已经追剧
pub async fn is_followed(conn: &Conn, user_id: i32, movie_id: i32) -> anyhow::Result<bool> {
    let row = conn
        .query_opt(
            r#" select id from follow_records where user_id = $1 and movie_id = $2 limit 1 "#,
            &[&user_id, &movie_id],
        )
        .await?;

    if let Some(_) = row {
        Ok(true)
    } else {
        Ok(false)
    }
}

// 删除追剧记录
pub async fn delete_follow_record(
    conn: &Conn,
    user_id: i32,
    movie_id: i32,
) -> anyhow::Result<()> {
    conn.execute(
        r#" delete from follow_records where user_id = $1 and movie_id = $2 "#,
        &[&user_id, &movie_id],
    )
    .await?;
    Ok(())
}


#[derive(Debug, Clone, Serialize, Table)]
pub struct MovieFollow{
    pub id: i32,
    pub movie_name: String,
    pub cover: String,
    pub create_time: chrono::DateTime<Local>,
}

pub async fn follow_list(
    conn: &Conn,
    user_id: i32,
) -> anyhow::Result<Vec<MovieFollow>> {
    // 列出追剧历史，从 follow_records 查询 创建时间， 然后根据 movie_id 分组, 查出最后插入的 video_id，和 create_time， 再从 movies 查出 name 作为 movie_name, cover, 从 videos 查出 name 作为 video_name  最后根据 follow_records中create_time 倒序排列，并分页
    let rows = conn
        .query(
            r#" select m.id, m.name as movie_name, m.cover, max(r.create_time) as create_time from follow_records as r left join movies as m on r.movie_id = m.id where r.user_id = $1 group by m.id, m.name, m.cover order by max(r.create_time) desc"#,
            &[&user_id],
        )
        .await?;
        
    Ok(rows.iter().map(|row| row.try_into().unwrap()).collect())
}
