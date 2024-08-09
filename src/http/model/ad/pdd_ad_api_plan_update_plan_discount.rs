//! 更新分时折扣

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Discounts {
    /// 小时。0-23分别表示第1个小时到第24个小时。
    pub index: i32,
    /// 折扣比例。千分比（即rate等于1000表示比例100%）。
    pub rate: i32,
}

#[derive(Debug, Serialize)]
pub struct PlanDiscount {
    /// 分时折扣配置列表
    pub discounts: Vec<Discounts>,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanUpdatePlanDiscountRequest {
    /// 分时折扣
    #[serde(rename = "planDiscount")]
    pub plan_discount: PlanDiscount,
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: i64,
}

impl RequestType for PddAdApiPlanUpdatePlanDiscountRequest {
    type Response = PddAdApiPlanUpdatePlanDiscountResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.update.plan.discount"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanUpdatePlanDiscountResponse {
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
