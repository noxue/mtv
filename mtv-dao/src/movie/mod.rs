
pub mod video;



use anyhow::anyhow;
use chrono::Local;
use serde::{Deserialize, Serialize,};
use sqlrs::{Conn, Table};

use crate::Page;

#[derive(Debug, Clone, Serialize, Table)]
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub cover: Option<String>,
    pub total: Option<i32>,
    pub description: Option<String>,
    pub is_top: Option<bool>,
    pub is_hot: Option<bool>,
    pub tags: Vec<String>,
    pub price_total: i32,
    pub is_show: bool,
    pub view: i32,
    pub likes: i32,
    pub vlikes: i32,
    pub is_finish: bool,
    pub share_title: Option<String>,
    pub share_pic: Option<String>,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
}

// 添加
pub async fn add(
    conn: &Conn,
    name: String,
    cover: String,
    description: String,
    tags: Vec<String>,
    price_total: i32,
    vlikes: i32,
) -> anyhow::Result<Movie> {
    let row = conn
        .query_one(
            r#" insert into movies (name, cover, description, tags, price_total, vlikes) values ($1, $2, $3, $4, $5, $6) returning * "#,
            &[&name, &cover, &description, &tags, &price_total, &vlikes],
        )
        .await.map_err(|e|{
            if e.to_string().contains("duplicate key value violates unique constraint \"movies_name_uindex\""){
                anyhow::anyhow!("电影名已存在")
            }else{
                anyhow::anyhow!("{}", e)
            }
        })?;
    Ok(row.try_into()?)
}

// 获取
pub async fn get(conn: &Conn, id: i32) -> anyhow::Result<Movie> {
    let row = conn
        .query_opt(r#" select * from movies where id = $1 "#, &[&id])
        .await?;
    if row.is_none() {
        return Err(anyhow!("电影不存在"));
    }
    Ok(row.unwrap().try_into()?)
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMovie {
    pub name: String,
    pub cover: Option<String>,
    pub total: Option<i32>,
    pub description: Option<String>,
    pub is_top: Option<bool>,
    pub is_hot: Option<bool>,
    pub tags: Vec<String>,
    pub price_total: i32,
    pub is_show: bool,
    pub view: i32,
    pub vlikes: i32,
    pub is_finish: bool,
    pub share_title: Option<String>,
    pub share_pic: Option<String>,
}
// 更新
pub async fn update(conn: &Conn, id: i32, update_movie: &UpdateMovie) -> anyhow::Result<Movie> {
    let UpdateMovie {
        name,
        cover,
        total,
        description,
        is_top,
        is_hot,
        tags,
        price_total,
        is_show,
        view,
        vlikes,
        is_finish,
         share_title,
        share_pic,
    } = update_movie;
    let update_time = Local::now();

    let row = conn
        .query_one(
            r#" update movies set name = $1, cover = $2, total = $3, description = $4, is_top = $5, is_hot = $6, tags = $7, price_total = $8, is_show = $9, view = $10, vlikes = $11, is_finish = $12, share_title = $13, share_pic = $14, update_time = $15 where id = $16 returning * "#,
            &[&name, &cover, &total, &description, &is_top, &is_hot, &tags, &price_total, &is_show, &view, &vlikes, &is_finish, &share_title, &share_pic, &update_time, &id],
        )
        .await.map_err(|e|{
            if e.to_string().contains("duplicate key value violates unique constraint \"movies_name_uindex\""){
                anyhow::anyhow!("电影名已存在")
            
            }else if e.to_string().contains("unexpected number of rows"){
                anyhow::anyhow!("电影不存在")
            }else{
                anyhow::anyhow!("{}", e)
            }
        })?;

    Ok(row.try_into()?)
}

// 增加 观看 次数
pub async fn add_view(conn: &Conn, movie_id: i32, video_id:i32) -> anyhow::Result<()> {
    let r = conn.execute(r#" update movies set view = view + 1 where id = $1 "#, &[&movie_id])
        .await?;
    if r == 0 {
        return Err(anyhow!("电影不存在"));
    }
    // 更新视频的播放
    let r = conn.execute(r#" update videos set view = view + 1 where id = $1 "#, &[&video_id])
        .await?;
    if r == 0 {
        return Err(anyhow!("视频不存在"));
    }
    Ok(())
}


/// 删除
pub async fn delete(conn: &Conn, id: i32) -> anyhow::Result<()> {
    let r = conn.execute(r#" delete from movies where id = $1 "#, &[&id])
        .await?;
    if r == 0 {
        return Err(anyhow!("电影不存在"));
    }
    Ok(())
}


// 列表，支持顺序和倒序
pub async fn list(conn: &Conn, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<Movie>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    let sql = format!("select * from movies order by id {} limit $1 offset $2",desc);

    let rows = conn
        .query(
            &sql,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;
    
    let movies: Vec<Movie> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies",
            &[],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}

// let movies = mtv_dao::movie::list_tags(&conn, tags.unwrap(), desc.is_some(), page, page_size).await?;
pub async fn list_tags(conn: &Conn, tags: Vec<String>, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<Movie>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    let sql = format!("select * from movies where tags @> $1 order by id {} limit $2 offset $3",desc);

    let rows = conn
        .query(
            &sql,
            &[&tags, &size, &(&(page - 1) * size)],
        )
        .await?;
    
    let movies: Vec<Movie> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies where tags @> $1",
            &[&tags],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}

pub async fn list_hot(conn: &Conn, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<Movie>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    let sql = format!("select * from movies where is_hot = true order by update_time {}  limit $1 offset $2",desc);

    let rows = conn
        .query(
            &sql,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;
    
    let movies: Vec<Movie> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies where is_hot = true",
            &[],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}


pub async fn list_top(conn: &Conn, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<Movie>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    let sql = format!("select * from movies where is_top = true order by update_time {}  limit $1 offset $2",desc);

    let rows = conn
        .query(
            &sql,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;
    
    let movies: Vec<Movie> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies where is_top = true",
            &[],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}


pub async fn list_like(conn: &Conn, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<Movie>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    let sql = format!("select * from movies where is_show = true order by likes {}  limit $1 offset $2",desc);

    let rows = conn
        .query(
            &sql,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;
    
    let movies: Vec<Movie> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies where is_show = true",
            &[],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}


pub async fn list_view(conn: &Conn, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<Movie>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    let sql = format!("select * from movies order by view {}  limit $1 offset $2",desc);

    let rows = conn
        .query(
            &sql,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;
    
    let movies: Vec<Movie> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies",
            &[],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}


pub async fn list_update_time(conn: &Conn, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<Movie>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    let sql = format!("select * from movies order by update_time {}  limit $1 offset $2",desc);

    let rows = conn
        .query(
            &sql,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;
    
    let movies: Vec<Movie> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies",
            &[],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}


#[derive(Debug, Clone, Serialize, Table)]
pub struct MovieWithScore {
    pub id: i32,
    pub name: String,
    pub cover: Option<String>,
    pub total: Option<i32>,
    pub description: Option<String>,
    pub is_top: Option<bool>,
    pub is_hot: Option<bool>,
    pub tags: Vec<String>,
    pub price_total: i32,
    pub is_show: bool,
    pub view: i32,
    pub likes: i32,
    pub vlikes: i32,
    pub is_finish: bool,
    pub share_title: Option<String>,
    pub share_pic: Option<String>,
    pub create_time: chrono::DateTime<Local>,
    pub update_time: chrono::DateTime<Local>,
    pub score: Option<i32>,
}

// let movies = mtv_dao::movie::list_price(&conn, desc.is_some(), page, page_size).await?;
pub async fn list_price(conn: &Conn, desc: bool, page: i64, size: i64) -> anyhow::Result<Page<Vec<MovieWithScore>>> {

    let desc = if desc{ 
        "desc"
    } else{ 
        "asc"
    };

    /*
    CREATE TABLE consume_records (
    id serial PRIMARY KEY,
    user_id int4 NOT NULL,
    movie_id int4 NOT NULL,
    movie_part_id int4 NOT NULL,
    score int4 NOT NULL DEFAULT 0,
    mark varchar(255) NOT NULL,
    create_time timestamp with time zone DEFAULT now()
);
     */
    // 根据 consume_records 中 根据 movie_id 分组，然后求和 score
    let sql = format!("select  movies.*, sum(consume_records.score) as score from movies left join consume_records on movies.id = consume_records.movie_id group by movies.id order by score {}  limit $1 offset $2",desc);

    let rows = conn
        .query(
            &sql,
            &[&size, &(&(page - 1) * size)],
        )
        .await?;

    let movies: Vec<MovieWithScore> = rows.iter().map(|row| row.try_into().unwrap()).collect();

    let total = conn
        .query_one(
            "select count(*) from movies",
            &[],
        )
        .await?;

    let total: i64 = total.get(0);
    Ok(
        Page {
            total,
            page,
            size,
            data: movies,
        }
    )
}

// 修改点赞数  likes=likes+likes
pub async fn update_likes(conn: &Conn, id: i32, likes: i32) -> anyhow::Result<()> {
    let r = conn.execute(r#" update movies set likes = likes + $1 where id = $2 "#, &[&likes, &id])
        .await?;
    if r == 0 {
        return Err(anyhow!("点赞失败"));
    }
    Ok(())
}
