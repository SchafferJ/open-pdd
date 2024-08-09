//! 更新单元出价

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitUpdateUnitBidRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 出价不得为空。单位厘。
    pub bid: i64,
}

impl RequestType for PddAdApiUnitUpdateUnitBidRequest {
    type Response = PddAdApiUnitUpdateUnitBidResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.update.unit.bid"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitUpdateUnitBidResponse {
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
