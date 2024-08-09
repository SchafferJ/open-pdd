//! # 获取可发货快递接口
//!
//! 给商家提供可发货的快递公司，此数据仅作参考，如返回为空不代表不可发货
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsAvailableCompanyRecommendRequest {
    /// 收件人市id（最多支持50个）
    pub city_id: Vec<i64>,
}

impl RequestType for PddLogisticsAvailableCompanyRecommendRequest {
    type Response = PddLogisticsAvailableCompanyRecommendResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.available.company.recommend"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 收件人市id
    #[serde(default)]
    pub city_id: i64,
    /// 快递公司id
    #[serde(default)]
    pub company_id: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsAvailableCompanyRecommendResponse {
    #[serde(default)]
    pub result: Option<Vec<Result>>,
}
