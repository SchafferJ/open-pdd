//! 更新全站推广成交出价

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrUpdateOptimizationBidRequest {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// 成交出价，4000~1000000(表示4-1000元)
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: i64,
}

impl RequestType for PddAdApiUnitTrUpdateOptimizationBidRequest {
    type Response = PddAdApiUnitTrUpdateOptimizationBidResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.update.optimization.bid"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrUpdateOptimizationBidResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否更新成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
