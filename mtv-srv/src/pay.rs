

// let pay = WxPay::new(
//     "1559638011",
//     &key,
//     "3CD22EDD308C67AB6A52FEB55A424AD4BB98254B",
//     "3A252DB28DA685467AD80365E87DB041",
//     "https://dot2.com/notify",
// );

use lazy_static::lazy_static;
use mtv_config::CONFIG;
use wxpay::WxPay;
pub use wxpay::WxPayNotify;


lazy_static!(
    pub static ref PAY: WxPay = WxPay::new(
        &CONFIG.wx_pay_mch_id,
        &CONFIG.wx_pay_cert_key,
        &CONFIG.wx_pay_serial_no,
        &CONFIG.wx_pay_api_v3_private_key,
        &CONFIG.wx_pay_notify_url,
    );
);