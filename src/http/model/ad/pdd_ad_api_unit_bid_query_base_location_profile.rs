//! 获取可用资源位

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitBidQueryBaseLocationProfileRequest;

impl RequestType for PddAdApiUnitBidQueryBaseLocationProfileRequest {
    type Response = PddAdApiUnitBidQueryBaseLocationProfileResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.bid.query.base.location.profile"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 资源位描述
    #[serde(default, rename = "locationName")]
    pub location_name: String,
    /// 资源位类型
    #[serde(default, rename = "locationType")]
    pub location_type: i32,
    /// 资源位预估人群
    #[serde(default, rename = "pvString")]
    pub pv_string: String,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitBidQueryBaseLocationProfileResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
