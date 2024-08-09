//! 更新计划名称

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanUpdatePlanNameRequest {
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: i64,
    /// 计划名称
    #[serde(rename = "planName")]
    pub plan_name: String,
}

impl RequestType for PddAdApiPlanUpdatePlanNameRequest {
    type Response = PddAdApiPlanUpdatePlanNameResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.update.plan.name"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanUpdatePlanNameResponse {
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
