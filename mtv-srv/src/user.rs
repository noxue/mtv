use crate::Result;
use anyhow::Context;
use mtv_config::CONFIG;
use mtv_dao::Db;
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::REDIS;

pub async fn down_up() -> Result<()> {
    mtv_dao::down().await;
    mtv_dao::up().await;
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct LoginResult {
    pub token: String,
    pub openid: String,
    pub unionid: String,
}

pub async fn login(appid: &str, code: &str, login_type: &str) -> Result<LoginResult> {
    let (openid, unionid) = match login_type {
        "weapp" => login_weapp(appid, code).await?,
        "mp" => login_mp(code).await?,
        _ => {
            return Err("登录类型不支持".into());
        }
    };

    if openid.is_empty() && unionid.is_empty() {
        return Err("openid和unionid都为空".into());
    }

    let conn = Db::get_conn();

    let user = match mtv_dao::user::get_by_unionid_or_openid(&conn, &unionid, &openid)
        .await
        .map_err(|e| {
            log::error!("根据unionid或openid获取用户出错:{:?}", e);
            "根据unionid或openid获取用户出错"
        })? {
        Some(user) => user,
        None => {
            let user = mtv_dao::user::create_user_by_unionid_and_openid(&conn, &unionid, &openid)
                .await
                .map_err(|e| {
                    log::error!("根据unionid或openid创建用户出错:{:?}", e);
                    "根据unionid或openid创建用户出错"
                })?;
            user
        }
    };

    let token = gen_token(user.id).await?;

    log::debug!("user:{:?}", user);

    Ok(LoginResult {
        token,
        openid,
        unionid,
    })
}

async fn gen_token(user_id: i32) -> Result<String> {
    let token_key = format!("user_token_{}", user_id);

    let token = match REDIS
        .get_connection()
        .map_err(|e| {
            log::error!("获取redis连接出错:{:?}", e);
            "获取redis连接出错"
        })?
        .get(&token_key)
        .context("从redis获取token出错")?
    {
        Some(v) => v,
        None => {
            let token = uuid::Uuid::new_v4().to_string().replace("-", "");
            REDIS
                .get_connection()
                .context("获取redis连接出错")?
                .set_ex::<&str, String, ()>(&token_key, token.clone(), 60 * 60 * 24 * 30)
                .context("设置token出错")?;

            // 以user_id_{token} 为key 保存用户id，用在中间件中 根据token获取uid
            let key = format!("user_id_{}", token);
            REDIS
                .get_connection()
                .context("获取redis连接出错")?
                .set_ex::<String, i32, ()>(key, user_id, 60 * 60 * 24 * 30)
                .context("设置uid出错")?;
            token
        }
    };
    log::debug!("redis:{:?}", token);
    Ok(token)
}

#[derive(Deserialize, Debug)]
struct WeappLoginResponse {
    openid: Option<String>,
    session_key: Option<String>,
    unionid: Option<String>,
    errcode: Option<i32>,
    errmsg: Option<String>,
}
async fn login_weapp(appid: &str, code: &str) -> Result<(String, String)> {
    let weapp = CONFIG
        .get_weapp(appid)
        .ok_or(anyhow::anyhow!("appid:{} 未配置", appid))?;

    let url = format!(
        "https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
        appid=weapp.id,
        secret=weapp.secret,
        code=code
    );
    let res = reqwest::get(url)
        .await
        .context("根据code请求微信服务器出错")?;

    let data: WeappLoginResponse = res
        .json()
        .await
        .context("根据code获取openid 解析json出错")?;

    log::debug!("{:?}", data);

    if let Some(errcode) = data.errcode {
        if errcode != 0 {
            return Err(format!("微信登录出错:{}", data.errmsg.unwrap()).into());
        }
    }

    Ok((
        data.openid.unwrap_or_default(),
        data.unionid.unwrap_or_default(),
    ))
}

#[derive(Deserialize, Debug)]
struct MpLoginResponse {
    access_token: Option<String>,
    expires_in: Option<i32>,
    refresh_token: Option<String>,
    openid: Option<String>,
    scope: Option<String>,
    is_snapshotuser: Option<i32>,
    unionid: Option<String>,
    errcode: Option<i32>,
    errmsg: Option<String>,
}

async fn login_mp(code: &str) -> Result<(String, String)> {
    let url = format!(
            "https://api.weixin.qq.com/sns/oauth2/access_token?appid={appid}&secret={secret}&code={code}&grant_type=authorization_code",
            appid=CONFIG.wx_mp_app_id,
            secret=CONFIG.wx_mp_app_secret,
            code=code,
        );
    let res = reqwest::get(url).await.context("请求微信登录接口出错")?;
    let data: MpLoginResponse = res.json().await.context("解析json数据出错")?;

    log::debug!("{:?}", data);

    if let Some(errcode) = data.errcode {
        if errcode != 0 {
            return Err(format!("微信登录出错:{}", data.errmsg.unwrap()).into());
        }
    }

    Ok((
        data.openid.unwrap_or_default(),
        data.unionid.unwrap_or_default(),
    ))
}

// 设置手机号和密码
pub async fn set_phone_and_password(uid: i32, phone: &str, password: &str) -> Result<()> {
    let conn = Db::get_conn();
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
        log::error!("密码加密出错:{:?}", e);
        "密码加密出错"
    })?;
    log::debug!("password:{}", password);
    mtv_dao::user::set_phone_and_password(&conn, uid, phone, &password)
        .await
        .map_err(|e| {
            log::error!("设置手机号和密码出错:{:?}", e);
            "设置手机号和密码出错"
        })?;
    Ok(())
}

// 根据手机号密码登录
pub async fn login_phone(phone: &str, password: &str) -> Result<String> {
    let conn = Db::get_conn();
    let user = mtv_dao::user::get_by_phone(&conn, phone).await?;
    let encoded_password = user.auth.password.as_ref().ok_or("密码为空")?;
    let is_match = bcrypt::verify(password, encoded_password).map_err(|e| {
        log::error!("密码验证出错:{:?}", e);
        "密码验证出错"
    })?;

    if !is_match {
        return Err("密码错误".into());
    }

    let token = gen_token(user.id).await?;

    Ok(token)
}

/// 根据token获取uid
pub fn get_uid(token: &str) -> Result<i32> {
    let key = format!("user_id_{}", token);
    let uid: Option<i32> = REDIS
        .get_connection()
        .context("获取token链接出错")?
        .get(&key)
        .context("根据key获取uid出错")?;
    Ok(uid.ok_or("token已过期")?)
}

/// 根据uid查询用户信息
pub async fn get(uid: i32) -> Result<impl Serialize> {
    let conn = Db::get_conn();
    let user = mtv_dao::user::get(&conn, uid)
        .await
        .context("根据uid获取用户信息出错")?;

    Ok(json!({
        "id": user.id,
        "nickname": user.nickname,
        "avatar": user.avatar,
        "score": user.score,
        "vip": user.vip,
        "vip_expire_time": user.vip_expire_time,
    }))
}

pub async fn set_channel(uid: i32, channel: &str) -> Result<()> {
    let conn = Db::get_conn();
    mtv_dao::user::set_channel(&conn, uid, channel)
        .await
        .context("设置用户渠道出错")?;
    Ok(())
}

// 分页列出用户
pub async fn list(page: i64, size: i64) -> Result<impl Serialize> {
    let conn = Db::get_conn();
    let users = mtv_dao::user::list(&conn, page, size).await?;

    Ok(users)
}

// 根据渠道列出用户
pub async fn list_by_channel(channel: &str, page: i64, size: i64) -> Result<impl Serialize> {
    let conn = Db::get_conn();
    let users = mtv_dao::user::list_by_channel(&conn, channel, page, size).await?;

    Ok(users)
}
