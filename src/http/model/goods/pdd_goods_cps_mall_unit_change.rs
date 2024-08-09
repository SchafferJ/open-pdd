//! # 修改全店推广API
//!
//! 修改全店推广计划
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCpsMallUnitChangeRequest {
    /// 全店推广计划佣金比（千分比）
    pub rate: i32,
}

impl RequestType for PddGoodsCpsMallUnitChangeRequest {
    type Response = PddGoodsCpsMallUnitChangeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cps.mall.unit.change"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCpsMallUnitChangeResponse {
    /// 是否成功
    #[serde(default)]
    pub result: bool,
}
