//! 切换计划至ocpc

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OptimizationMessage {
    /// 智能投放期出价
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: i64,
    /// 优化目标。单元使用OCPC功能时，该值必须传2。
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: i32,
    /// 优化方式。当使用OCPC时，该值必须传2。
    #[serde(rename = "optimizationMethod")]
    pub optimization_method: i32,
}

#[derive(Debug, Serialize)]
pub struct AdUnitUpdateOcpcMessage {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// OCPC信息
    #[serde(rename = "optimizationMessage")]
    pub optimization_message: OptimizationMessage,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanUpdatePlanToOcpcRequest {
    /// 单元OCPC信息列表
    #[serde(rename = "adUnitUpdateOcpcMessageList")]
    pub ad_unit_update_ocpc_message_list: Vec<AdUnitUpdateOcpcMessage>,
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: i64,
    /// 场景类型。0表示搜索。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiPlanUpdatePlanToOcpcRequest {
    type Response = PddAdApiPlanUpdatePlanToOcpcResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.update.plan.to.ocpc"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanUpdatePlanToOcpcResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
