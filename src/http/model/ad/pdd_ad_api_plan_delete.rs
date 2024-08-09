//! 删除计划

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanDeleteRequest {
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: i64,
    /// 场景类型，0-搜索，2-展示
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiPlanDeleteRequest {
    type Response = PddAdApiPlanDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.delete"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanDeleteResponse {
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
