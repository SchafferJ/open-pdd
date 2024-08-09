//! 查询广告主详情信息

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiAdvertiserQueryDetailRequest;

impl RequestType for PddAdApiAdvertiserQueryDetailRequest {
    type Response = PddAdApiAdvertiserQueryDetailResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.advertiser.query.detail"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct AdvertiserPunishDetail {
    /// 惩罚原因
    #[serde(default)]
    pub punish_reason: String,
    /// 惩罚状态。1表示正常，2表示惩罚中。
    #[serde(default)]
    pub punish_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 广告主审核状态。1表示审核通过，2表示待审核，3表示审核拒绝，4表示复审拒绝。
    #[serde(default)]
    pub advertiser_audit_status: i32,
    /// 惩罚信息
    #[serde(default)]
    pub advertiser_punish_detail: Option<AdvertiserPunishDetail>,
    /// 广告主审核原因
    #[serde(default)]
    pub audit_reason: i32,
    #[serde(default, rename = "mall_Id")]
    pub mall_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiAdvertiserQueryDetailResponse {
    #[serde(default)]
    pub error_code: i32,
    #[serde(default)]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
