//! 更新单个定向/资源位

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AdUnitBids {
    /// 出价Id
    #[serde(rename = "bidId")]
    pub bid_id: i64,
    /// 出价，万分比
    #[serde(rename = "bidValue")]
    pub bid_value: i64,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitBidUpdateRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 更新列表
    #[serde(rename = "adUnitBids")]
    pub ad_unit_bids: Vec<AdUnitBids>,
    /// 出价资源类型。1表示人群定向，2表示资源位。
    #[serde(rename = "bidReferenceType")]
    pub bid_reference_type: i32,
}

impl RequestType for PddAdApiUnitBidUpdateRequest {
    type Response = PddAdApiUnitBidUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.bid.update"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitBidUpdateResponse {
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
