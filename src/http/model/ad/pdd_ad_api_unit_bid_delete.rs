//! 删除定向/资源位

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitBidDeleteRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 出价Id列表
    #[serde(rename = "bidIds")]
    pub bid_ids: Vec<i64>,
}

impl RequestType for PddAdApiUnitBidDeleteRequest {
    type Response = PddAdApiUnitBidDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.bid.delete"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitBidDeleteResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否删除成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
