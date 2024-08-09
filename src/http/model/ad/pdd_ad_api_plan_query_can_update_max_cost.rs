//! 当天是否允许计划的推广预算

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanQueryCanUpdateMaxCostRequest {
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: i64,
}

impl RequestType for PddAdApiPlanQueryCanUpdateMaxCostRequest {
    type Response = PddAdApiPlanQueryCanUpdateMaxCostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.query.can.update.max.cost"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanQueryCanUpdateMaxCostResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否可以更新
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
