//! 删除单元

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitDeleteRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 场景类型。0表示搜索，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiUnitDeleteRequest {
    type Response = PddAdApiUnitDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.delete"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitDeleteResponse {
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
