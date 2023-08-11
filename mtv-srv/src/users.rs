use anyhow::{Context, Ok};
use mtv_config::CONFIG;
use mtv_dao::Db;
use redis::Commands;
use serde::Deserialize;

use crate::utils::REDIS;

pub async fn login(code: &str, login_type: &str) -> anyhow::Result<String> {
    let (openid, unionid) = match login_type {
        "weapp" => login_weapp(code).await.context("小程序登录出错"),
        "mp" => login_mp(code).await.context("公众号登录出错"),
        _ => {
            anyhow::bail!("登录类型不支持")
        }
    }?;

    if openid.is_empty() && unionid.is_empty() {
        anyhow::bail!("openid和unionid都为空");
    }

    let conn = Db::get_conn();

    let user = match mtv_dao::user::get_by_unionid_or_openid(&conn, &unionid, &openid)
        .await
        .context("根据unionid或openid获取用户出错")?
    {
        Some(user) => user,
        None => {
            let user = mtv_dao::user::create_user_by_unionid_and_openid(&conn, &unionid, &openid)
                .await
                .context("根据unionid或openid创建用户出错")?;
            user
        }
    };

    let token_key = format!("user_token_{}", user.id);
    

    let token = match REDIS
        .get_connection()
        .context("获取redis连接出错")?
        .get(&token_key)
        .context("从redis获取token出错")?
    {
        Some(v) => v,
        None => {
            let token = uuid::Uuid::new_v4().to_string();
            REDIS
                .get_connection()
                .context("获取redis连接出错")?
                .set_ex::<&str, String, ()>(&token_key, token.clone(), 60 * 60 * 24 * 30).context("设置token出错")?;
            token
        }
    };
    log::debug!("redis:{:?}", token);

    log::debug!("user:{:?}", user);

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
async fn login_weapp(code: &str) -> anyhow::Result<(String, String)> {
    let url = format!(
        "https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
        appid=CONFIG.weapp_appid,
        secret=CONFIG.weapp_secret,
        code=code
    );
    let res = reqwest::get(url).await?;

    let data: WeappLoginResponse = res.json().await?;

    log::debug!("{:?}", data);

    if let Some(errcode) = data.errcode {
        if errcode != 0 {
            return Err(anyhow::anyhow!("微信登录出错:{}", data.errmsg.unwrap()));
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

async fn login_mp(code: &str) -> anyhow::Result<(String, String)> {
    let url = format!(
            "https://api.weixin.qq.com/sns/oauth2/access_token?appid={appid}&secret={secret}&code={code}&grant_type=authorization_code",
            appid=CONFIG.weapp_appid,
            secret=CONFIG.weapp_secret,
            code=code,
        );
    let res = reqwest::get(url).await.context("请求微信登录接口出错")?;
    let data: MpLoginResponse = res.json().await.context("解析json数据出错")?;

    log::debug!("{:?}", data);

    if let Some(errcode) = data.errcode {
        if errcode != 0 {
            return Err(anyhow::anyhow!("微信登录出错:{}", data.errmsg.unwrap()));
        }
    }

    Ok((
        data.openid.unwrap_or_default(),
        data.unionid.unwrap_or_default(),
    ))
}

#[cfg(test)]
mod tests {
    use std::env;

    use redis::Commands;

    use crate::utils::REDIS;

    #[test]
    fn test_redis() {
        env::set_var("REDIS_URL", "redis://localhost:6379/0");
        let token_key = format!("user_token_{}", 1);
        let token = uuid::Uuid::new_v4().to_string();

        // 获取数据
        let s: Option<String> = REDIS.get_connection().unwrap().get(&token_key).unwrap();
        dbg!(s);

        REDIS
            .get_connection()
            .unwrap()
            .set_ex::<&str, String, i32>(&token_key, token, 10);
    }
}
