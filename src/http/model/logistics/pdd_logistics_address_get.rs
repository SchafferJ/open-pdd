//! # 获取拼多多标准地址库
//!
//! 获取拼多多标准地址库
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsAddressGetRequest;

impl RequestType for PddLogisticsAddressGetRequest {
    type Response = PddLogisticsAddressGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.address.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct LogisticsAddress {
    /// 地区ID
    #[serde(default)]
    pub id: i64,
    /// 是否有效，0-无效，1-有效
    #[serde(default)]
    pub is_enabled: i32,
    /// 地区邮编
    #[serde(default)]
    pub national_code: String,
    /// 父地区ID，顶点id为0
    #[serde(default)]
    pub parent_id: i64,
    /// 地区名称
    #[serde(default)]
    pub region_name: String,
    /// 地区层级，1-省份，2-市级，3-区级
    #[serde(default)]
    pub region_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsAddressGetResponse {
    /// 地址对象列表
    #[serde(default)]
    pub logistics_address_list: Option<Vec<LogisticsAddress>>,
}
