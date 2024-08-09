//! 同步定向/资源位

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AreaStruct {
    /// 地域Id列表。具体地域Id编码参见接口返回：pdd.ad.api.unit.bid.query.targeting.tag.list
    #[serde(rename = "areaIds")]
    pub area_ids: Option<Vec<i32>>,
}

#[derive(Debug, Serialize)]
pub struct AdTargetingSet {
    /// 地域定向
    #[serde(rename = "areaStruct")]
    pub area_struct: Option<AreaStruct>,
}

#[derive(Debug, Serialize)]
pub struct AdTargetingVO {
    /// 定向集合
    #[serde(rename = "adTargetingSet")]
    pub ad_targeting_set: Option<AdTargetingSet>,
    /// 定向名称
    #[serde(rename = "targetingName")]
    pub targeting_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdUnitBids {
    /// 定向信息。仅支持地域定向。
    #[serde(rename = "adTargetingVO")]
    pub ad_targeting_vo: Option<AdTargetingVO>,
    /// 可选人群定向类型或者可选资源位定向类型。人群定向类型，可用枚举值，参考接口：pdd.ad.api.unit.bid.query.base.target.profile资源位定向类型，可用枚举值，参考接口：pdd.ad.api.unit.bid.query.available.location
    #[serde(rename = "bidReferenceId")]
    pub bid_reference_id: i64,
    /// 出价，万分比，10000表示100%
    #[serde(rename = "bidValue")]
    pub bid_value: i64,
    /// 二级定向Id。默认为0。
    #[serde(rename = "subBidReferenceId")]
    pub sub_bid_reference_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitBidSyncRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 出价信息列表
    #[serde(rename = "adUnitBids")]
    pub ad_unit_bids: Vec<AdUnitBids>,
    /// 出价资源类型。1表示人群定向，2表示资源位。
    #[serde(rename = "bidReferenceType")]
    pub bid_reference_type: i32,
}

impl RequestType for PddAdApiUnitBidSyncRequest {
    type Response = PddAdApiUnitBidSyncResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.bid.sync"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitBidSyncResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否同步成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
