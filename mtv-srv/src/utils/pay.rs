/*
//         // 从 config.wx_pay.cert_key_path 文件中读取数据
//         let mut private_key = "".to_string();
//         let mut file = File::open(config.wx_pay.cert_key_path).unwrap();
//         file.read_to_string(&mut private_key).unwrap();
//         WxPay {
//             api_host: config.wx_pay.api_host,
//             appid: config.wx_mp.app_id,
//             mchid: config.wx_pay.mch_id,
//             mch_certificate_serial_number: config.wx_pay.serial_no,
//             mch_api_v3_key: config.wx_pay.api_v3_private_key,
//             private_key,
//             description: "".to_string(),
//             out_trade_no: "".to_string(),
//             attach: "".to_string(),
//             notify_url: config.wx_pay.notify_url,
//             amount: 0,
//             openid: "".to_string(),
//         }
 */
lazy_static::lazy_static! {
    pub static ref WX_PAY: WxPay = WxPay{
        api_host: CONFIG.wx_pay_api_host.clone(),
        appid: CONFIG.wx_mp_app_id.clone(),
        mchid: CONFIG.wx_pay_mch_id.clone(),
        mch_certificate_serial_number: CONFIG.wx_pay_serial_no.clone(),
        mch_api_v3_key: CONFIG.wx_pay_api_v3_private_key.clone(),
        private_key: "".to_string(),
        description: "".to_string(),
        out_trade_no: "".to_string(),
        attach: "".to_string(),
        notify_url: CONFIG.wx_pay_notify_url.clone(),
        amount: 0,
        openid: "".to_string(),
    };
}

use crate::Result;
use aes_gcm::{
    aead::{Aead, Payload},
    Aes256Gcm, KeyInit,
};
use crypto::common::generic_array::GenericArray;
use mtv_config::CONFIG;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Payer {
    pub openid: String,
}
/// 微信支付，回调解密
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WxNotifyData {
    pub mchid: String,
    pub appid: String,
    pub out_trade_no: String,
    pub transaction_id: String,
    pub trade_type: String,
    pub trade_state: String,
    pub trade_state_desc: String,
    pub bank_type: String,
    pub attach: String,
    pub success_time: String,
    pub payer: Payer,
    pub amount: WxNotifyDataAmount,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WxNotifyDataAmount {
    pub total: u32,
    pub payer_total: u32,
    pub currency: String,
    pub payer_currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxPayNotifyResource {
    pub algorithm: String,
    pub associated_data: String,
    pub ciphertext: String,
    pub nonce: String,
    pub original_type: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WxPayNotify {
    pub create_time: String,
    pub event_type: String,
    pub id: String,
    pub resource: WxPayNotifyResource,
    pub resource_type: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WxPay {
    api_host: String,
    appid: String,
    mchid: String,
    mch_certificate_serial_number: String,
    mch_api_v3_key: String,
    private_key: String,
    description: String,
    out_trade_no: String,
    attach: String,
    notify_url: String,
    amount: i32,
    openid: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct WxPrepayRes {
    code: i32,
    msg: String,
    data: Option<Value>,
}

impl WxPay {
    pub async fn prepay(
        &self,
        out_trade_no: String,
        amount: i32,
        openid: String,
        description: Option<String>,
        attach: Option<String>,
    ) -> Result<impl Serialize> {
        let body = WxPay {
            out_trade_no,
            amount,
            openid,
            description: description.unwrap_or_default(),
            attach: attach.unwrap_or_default(),
            ..self.clone()
        };

        // post 请求到 body.api_host
        let res = match reqwest::Client::new()
            .post(&body.api_host)
            .json(&body)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                log::error!("请求支付接口失败: {}", e);
                return Err("请求支付接口失败")?;
            }
        };

        let text = res.text().await.map_err(|e| {
            log::error!("请求支付接口失败: {}", e);
            "请求支付接口失败"
        })?;
        log::debug!("prepay res: {}", text);

        // 解析返回的数据
        let res = match serde_json::from_str::<WxPrepayRes>(&text) {
            Ok(res) => res,
            Err(e) => {
                log::error!("解析支付接口返回数据失败: {}", e);
                return Err("解析支付接口返回数据失败")?;
            }
        };

        log::debug!("prepay res: {:?}", res);

        Ok(res.data)
    }

    pub fn decode(&self, params: WxPayNotify) -> Result<WxNotifyData> {
        let auth_key_length = 16;

        let mut t_key = [0u8; 32];
        hex::decode_to_slice(hex::encode(&self.mch_api_v3_key), &mut t_key as &mut [u8]).map_err(
            |e| {
                log::error!("解析微信支付回调 key 失败: {}", e);
                "解析微信支付回调 key 失败"
            },
        )?;
        let key = GenericArray::from_slice(&t_key);

        let mut t_nonce = [0u8; 12];
        hex::decode_to_slice(
            hex::encode(params.resource.nonce.clone()),
            &mut t_nonce as &mut [u8],
        )
        .map_err(|e| {
            log::error!("解析微信支付回调 nonce 失败: {}", e);
            "解析微信支付回调 nonce 失败"
        })?;
        let nonce = GenericArray::from_slice(&t_nonce);

        let t_ciphertext_base =
            base64::decode(params.resource.ciphertext.clone()).map_err(|e| {
                log::error!("解析微信支付回调 ciphertext 失败: {}", e);
                "解析微信支付回调 ciphertext 失败"
            })?;
        let cipherdata_length = t_ciphertext_base.len() - auth_key_length;

        let cipherdata = &t_ciphertext_base[0..cipherdata_length];
        let auth_tag = &t_ciphertext_base[cipherdata_length..];

        let mut ciphertext = Vec::from(cipherdata);
        ciphertext.extend_from_slice(&auth_tag);

        let mut t_add = [0u8; 11]; // 这里可能会根据返回值 associated_data 长度而不同，目前应该是固定为 "transaction" 。
        hex::decode_to_slice(
            hex::encode(params.resource.associated_data.clone()),
            &mut t_add as &mut [u8],
        )
        .map_err(|e| {
            log::error!("解析微信支付回调 associated_data 失败: {}", e);
            "解析微信支付回调 associated_data 失败"
        })?;
        let payload = Payload {
            msg: &ciphertext,
            aad: &t_add,
        };
        let cipher = Aes256Gcm::new(key);
        let plaintext = match cipher.decrypt(nonce, payload) {
            Ok(v) => v,
            Err(e) => {
                log::error!("支付解密失败: {}", e);
                return Err("支付解密失败")?;
            }
        };
        let content = std::str::from_utf8(&plaintext).map_err(|e| {
            log::error!("解析微信支付回调数据转字符串失败: {}", e);
            "解析微信支付回调数据转字符串失败"
        })?;
        let data: WxNotifyData = serde_json::from_str(content).map_err(|e| {
            log::error!("解析微信支付回调数据失败: {}", e);
            "解析微信支付回调数据失败"
        })?;

        Ok(data)
    }
}

// impl From<Config> for WxPay {
//     fn from(config: Config) -> Self {
//         // 从 config.wx_pay.cert_key_path 文件中读取数据
//         let mut private_key = "".to_string();
//         let mut file = File::open(config.wx_pay.cert_key_path).unwrap();
//         file.read_to_string(&mut private_key).unwrap();
//         WxPay {
//             api_host: config.wx_pay.api_host,
//             appid: config.wx_mp.app_id,
//             mchid: config.wx_pay.mch_id,
//             mch_certificate_serial_number: config.wx_pay.serial_no,
//             mch_api_v3_key: config.wx_pay.api_v3_private_key,
//             private_key,
//             description: "".to_string(),
//             out_trade_no: "".to_string(),
//             attach: "".to_string(),
//             notify_url: config.wx_pay.notify_url,
//             amount: 0,
//             openid: "".to_string(),
//         }
//     }
// }
