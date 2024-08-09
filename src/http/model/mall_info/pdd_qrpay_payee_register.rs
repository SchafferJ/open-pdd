//! # 交易二维码-参数注册接口
//!
//! 交易二维码订单同店铺下需要记录订单来源的业务场景，可以将参数定义为门店、柜员、店员等
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Payee {
    /// 参数名，用于注册到名单，并生成对应URL
    pub payee: String,
}

#[derive(Debug, Serialize)]
pub struct PddQrpayPayeeRegisterRequest {
    /// 参数列表
    pub payee_list: Vec<Payee>,
}

impl RequestType for PddQrpayPayeeRegisterRequest {
    type Response = PddQrpayPayeeRegisterResponse;

    fn get_type(&self) -> &'static str {
        "pdd.qrpay.payee.register"
    }
}

#[derive(Debug, Deserialize)]
pub struct PayeeResult {
    /// 参数名
    #[serde(default)]
    pub payee: String,
    /// 参数id
    #[serde(default)]
    pub payee_id: i64,
    /// 交易二维码图片UR
    #[serde(default)]
    pub qr_image_url: String,
    /// 交易二维码页面URL
    #[serde(default)]
    pub qr_pay_page_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PddQrpayPayeeRegisterResponse {
    /// 参数注册结果
    #[serde(default)]
    pub payee_result_list: Option<Vec<PayeeResult>>,
}
