//! 更新智能创意

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreativeUpdateSmartCreativeRequest {
    /// 标题
    pub text: String,
    /// 创意单元Id
    #[serde(rename = "unitCreativeId")]
    pub unit_creative_id: i64,
}

impl RequestType for PddAdApiUnitCreativeUpdateSmartCreativeRequest {
    type Response = PddAdApiUnitCreativeUpdateSmartCreativeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.creative.update.smart.creative"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreativeUpdateSmartCreativeResponse {
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
