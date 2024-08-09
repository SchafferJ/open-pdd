//! # 快递公司查看接口
//!
//! 获取快递公司名称
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsCompaniesGetRequest;

impl RequestType for PddLogisticsCompaniesGetRequest {
    type Response = PddLogisticsCompaniesGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.companies.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct LogisticsCompanies {
    /// 是否有效，0-当前不支持的快递公司，1-支持的快递公司，注意：发货时必须入参支持的快递公司id，否则会报错
    #[serde(default)]
    pub available: i32,
    /// 快递公司编码
    #[serde(default)]
    pub id: i64,
    /// 快递公司名称
    #[serde(default)]
    pub logistics_company: String,
    /// 物流公司代码
    #[serde(default)]
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsCompaniesGetResponse {
    /// 快递公司列表对象
    #[serde(default)]
    pub logistics_companies: Option<Vec<LogisticsCompanies>>,
}
