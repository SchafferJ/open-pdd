//! # 卡券（电子）核销接口（平台生成卡密）
//!
//! 平台生成卡密类卡券核销
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddVoucherOtaCardVerificationRequest {
    /// 待核销的券码
    pub card_no: String,
    /// 核销门店id
    pub store_id: i64,
    /// 核销门店名称
    pub store_name: String,
    /// 拼多多订单编号
    pub order_sn: Option<String>,
}

impl RequestType for PddVoucherOtaCardVerificationRequest {
    type Response = PddVoucherOtaCardVerificationResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.ota.card.verification"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 券码
    #[serde(default)]
    pub card_no: String,
    /// 店铺编码
    #[serde(default)]
    pub mall_id: i64,
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
    /// 核销状态（1-未核销，2-已核销， 3-已过期，4-已销毁，99-核销中）
    #[serde(default)]
    pub status: i32,
    /// 门店编码
    #[serde(default)]
    pub store_id: i64,
    /// 门店名称
    #[serde(default)]
    pub store_name: String,
    /// 核销时间（yyyy-MM-dd HH:mm:ss格式）
    #[serde(default)]
    pub verification_time: String,
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherOtaCardVerificationResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
