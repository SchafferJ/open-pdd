//! # 限时限量购活动结束接口
//!
//! 结束已创建的限时限量购活动
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionLimitedActivityCancelRequest {
    /// 活动id
    pub detail_id: i64,
    /// 商品id
    pub goods_id: i64,
}

impl RequestType for PddPromotionLimitedActivityCancelRequest {
    type Response = PddPromotionLimitedActivityCancelResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.limited.activity.cancel"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionLimitedActivityCancelResponse;
