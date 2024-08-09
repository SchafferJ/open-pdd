//! 智能创意流量比例分配

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreativeDistributeFlowRateRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 流量分配比例。万分比
    #[serde(rename = "creativeFlowRate")]
    pub creative_flow_rate: i32,
}

impl RequestType for PddAdApiUnitCreativeDistributeFlowRateRequest {
    type Response = PddAdApiUnitCreativeDistributeFlowRateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.creative.distribute.flow.rate"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreativeDistributeFlowRateResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否分配成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
