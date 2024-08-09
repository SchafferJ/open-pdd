//! 更新日消耗上限

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanUpdateMaxCostRequest {
    /// 日消耗上限。单位厘。
    #[serde(rename = "maxCost")]
    pub max_cost: i64,
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: i64,
}

impl RequestType for PddAdApiPlanUpdateMaxCostRequest {
    type Response = PddAdApiPlanUpdateMaxCostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.update.max.cost"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanUpdateMaxCostResponse {
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
