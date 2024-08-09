//! 更新优化信息

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OptionalOptimizationBidMessage {
    /// 可选优化出价价格
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: i64,
    /// 可选优化出价目标。3表示优化店铺关注，4表示优化商品收藏，5表示优化询单
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: i32,
}

#[derive(Debug, Serialize)]
pub struct OptimizationMessage {
    /// 数据积累期出价。当使用OCPX时对该字段赋值。
    #[serde(rename = "accumulationBid")]
    pub accumulation_bid: Option<i64>,
    /// 智能投放期出价。当使用OCPX时对该字段赋值。
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
    /// 优化目标。0表示不优化。1表示优化ROI，2表示优化转化成本。当计划使用智能推广时，该值必须传1；当单元使用自动调价功能(ECPC)时，该值必须传1；当单元使用OCPC功能时，该值必须传2。
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: i32,
    /// 优化方式。0表示不优化，1表示ECPC，2表示OCPC。当计划使用智能推广时，该值必须传0；当单元使用ECPC时，该值必须传1；当使用OCPC时，该值必须传2。
    #[serde(rename = "optimizationMethod")]
    pub optimization_method: Option<i32>,
    /// 可选优化出价列表。当使用OCPX时对该字段赋值。
    #[serde(rename = "optionalOptimizationBidMessageList")]
    pub optional_optimization_bid_message_list: Option<Vec<OptionalOptimizationBidMessage>>,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitUpdateOptimizationMessageRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 优化信息
    #[serde(rename = "optimizationMessage")]
    pub optimization_message: OptimizationMessage,
}

impl RequestType for PddAdApiUnitUpdateOptimizationMessageRequest {
    type Response = PddAdApiUnitUpdateOptimizationMessageResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.update.optimization.message"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitUpdateOptimizationMessageResponse {
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
