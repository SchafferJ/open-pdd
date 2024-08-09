//! 删除创意

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreativeDeleteRequest {
    /// 创意单元Id
    #[serde(rename = "unitCreativeId")]
    pub unit_creative_id: i64,
}

impl RequestType for PddAdApiUnitCreativeDeleteRequest {
    type Response = PddAdApiUnitCreativeDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.creative.delete"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreativeDeleteResponse {
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
