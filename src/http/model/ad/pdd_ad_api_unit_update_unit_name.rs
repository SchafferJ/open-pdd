//! 更新单元名称

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitUpdateUnitNameRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 单元名称
    #[serde(rename = "adName")]
    pub ad_name: String,
}

impl RequestType for PddAdApiUnitUpdateUnitNameRequest {
    type Response = PddAdApiUnitUpdateUnitNameResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.update.unit.name"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitUpdateUnitNameResponse {
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
