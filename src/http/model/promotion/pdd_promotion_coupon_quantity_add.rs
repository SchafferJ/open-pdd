//! # 增加优惠券发行数量接口
//!
//! 增加优惠券发行数量接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionCouponQuantityAddRequest {
    /// 券批次ID
    pub batch_id: i64,
    /// 要增加的数量
    pub add_quantity: i64,
}

impl RequestType for PddPromotionCouponQuantityAddRequest {
    type Response = PddPromotionCouponQuantityAddResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.coupon.quantity.add"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionCouponQuantityAddResponse {
    /// 是否增加成功，true-成功，false-失败
    #[serde(default)]
    pub is_success: bool,
}
