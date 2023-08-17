use crate::Result;

pub async fn get_policy_token(
    api_host:&str,
    access_key_id:&str, 
    access_key_secret:&str,
    filename:&str,
    host:&str,
    callback_url:&str,
    bucket_name:&str,
    expire_time:i32,
) -> Result<String> {
    // reqwest 请求 http://127.0.0.1:5000/token?access_id=LTAI5tKvyLLhTVk2eanVhpcK&access_key=GfAXxTqvhHfFt1WoVa0H7yHlKNugyj&bucket_name=vlink-dot2&endpoint=https://oss-cn-hangzhou.aliyuncs.com&expire_time=60&host=https://vlink.static.noxue.com&callback_url=https://baidu.com&filename=tmp/asdfasfdasfd
    let url = format!("{}/token?access_id={}&access_key={}&bucket_name={}&expire_time={}&host={}&filename={}&callback_url={}",
    api_host,
                    access_key_id, 
                    access_key_secret,
                    bucket_name,
                    expire_time,
                    host,
                    filename,
                    callback_url
                );
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap();
    
    let ret = match client.get(url).send().await{
        Ok(ret) => ret,
        Err(e) => {
            log::error!("获取上传文件token出错:{}", e);
            return Err("获取上传文件token出错".into());
        }
    };

    if ret.status()!=200{
        log::error!("请求上传接口出错:{}", ret.status());
        return Err(format!("请求上传接口出错:{}", ret.status()).into());
    }

    let token = match ret.text().await{
        Ok(token) => token,
        Err(e) => {
            log::error!("获取上传文件token出错:{}", e);
            return Err("获取上传文件token出错".into());
        }
    };
    Ok(token)
}


// 复制文件
pub async fn copy(
    api_host:&str,
    access_key_id:&str, 
    access_key_key:&str,
    src_bucket_name:&str,
    dest_bucket_name:&str,
    endpoint:&str,
    src_object_name:&str,
    dest_object_name:&str,
) -> Result<String> {
    let url = format!(
        "{}/copy?access_id={}&access_key={}&src_bucket_name={}&dest_bucket_name={}&endpoint={}&src_object_name={}&dest_object_name={}",
        api_host,
        access_key_id, 
        access_key_key,
        src_bucket_name,
        dest_bucket_name,
        endpoint,
        src_object_name,
        dest_object_name
    );
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap();
    let ret = match client.get(url).send().await{
        Ok(ret) => ret,
        Err(e) => {
            log::error!("复制文件出错:{}", e);
            return Err("复制文件出错".into());
        }
    };

    if ret.status()!=200{
        log::error!("复制文件出错:{}", ret.status());
        return Err(format!("复制文件出错:{}", ret.status()).into());
    }
    let ret = match ret.text().await{
        Ok(ret) => ret,
        Err(e) => {
            log::error!("复制文件出错:{}", e);
            return Err("复制文件出错".into());
        }
    };
    
    log::debug!("ret:{}", ret);
    Ok(ret)

}


// 删除文件
pub async fn remove(
    api_host:&str,
    access_key_id:&str,
    access_key_key:&str,
    bucket_name:&str,
    endpoint:&str,
    object_name:&str,
) -> Result<String> {
    let url = format!(
        "{}/remove?access_id={}&access_key={}&bucket_name={}&endpoint={}&object_name={}",
        api_host,
        access_key_id,
        access_key_key,
        bucket_name,
        endpoint,
        object_name
    );
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap();
    let ret = match client.get(url).send().await{
        Ok(ret) => ret,
        Err(e) => {
            log::error!("删除文件出错:{}", e);
            return Err("删除文件出错".into());
        }
    };

    if ret.status()!=200{
        log::error!("删除文件出错:{}", ret.status());
        return Err(format!("删除文件出错:{}", ret.status()).into());
    }
    let ret = match ret.text().await{
        Ok(ret) => ret,
        Err(e) => {
            log::error!("删除文件出错:{}", e);
            return Err("删除文件出错".into());
        }
    };
    log::debug!("ret:{}", ret);
    Ok(ret)
}