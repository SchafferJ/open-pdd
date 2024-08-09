//! 校验是否能创建计划

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanQueryCanCreateAdPlanRequest {
    /// 单元名称
    #[serde(rename = "planName")]
    pub plan_name: String,
    /// 场景类型。0表示搜索，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiPlanQueryCanCreateAdPlanRequest {
    type Response = PddAdApiPlanQueryCanCreateAdPlanResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.query.can.create.ad.plan"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanQueryCanCreateAdPlanResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否可以创建
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
