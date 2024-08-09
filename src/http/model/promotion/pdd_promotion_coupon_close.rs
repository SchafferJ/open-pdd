//! # 关闭批次接口
//!
//! 关闭批次接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionCouponCloseRequest {
    /// 券批次ID
    pub batch_id: i64,
}

impl RequestType for PddPromotionCouponCloseRequest {
    type Response = PddPromotionCouponCloseResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.coupon.close"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionCouponCloseResponse {
    /// 是否关闭成功，true-成功，false-失败
    #[serde(default)]
    pub is_success: bool,
}
