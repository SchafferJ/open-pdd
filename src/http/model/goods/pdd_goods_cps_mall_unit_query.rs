//! # 查询全店推广API
//!
//! 查询全店推广计划
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCpsMallUnitQueryRequest;

impl RequestType for PddGoodsCpsMallUnitQueryRequest {
    type Response = PddGoodsCpsMallUnitQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cps.mall.unit.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCpsMallUnitQueryResponse {
    /// 全店推广计划佣金比（千分比）
    #[serde(default)]
    pub rate: i32,
    /// 修改后生效的佣金费率（千分比）
    #[serde(default)]
    pub rate_to_be: i32,
    /// 全店推广计划佣金生效时间
    #[serde(default)]
    pub rate_to_be_day: String,
    /// 全店推广计划状态：1-推广中，2-暂停推广
    #[serde(default)]
    pub status: i32,
    /// 修改后生效的状态，1-推广中，2-暂停，3-删除
    #[serde(default)]
    pub status_to_be: i32,
    /// 全店推广计划状态生效时间
    #[serde(default)]
    pub status_to_be_day: String,
}
