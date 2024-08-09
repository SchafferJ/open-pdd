//! # 服务市场订单履约查询
//!
//! 用于查询指定商家在服务市场订单执行履约的排序
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddServicemarketContractSearchRequest {
    /// 买家店铺id
    #[serde(rename = "mallId")]
    pub mall_id: i64,
}

impl RequestType for PddServicemarketContractSearchRequest {
    type Response = PddServicemarketContractSearchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.servicemarket.contract.search"
    }
}

#[derive(Debug, Deserialize)]
pub struct SpecValueResponse {
    /// 规格值id
    #[serde(default)]
    pub id: i64,
    /// 规格名称
    #[serde(default, rename = "specName")]
    pub spec_name: String,
    /// 规格值名称
    #[serde(default, rename = "specValue")]
    pub spec_value: String,
    /// 规格时长参数
    #[serde(default, rename = "timeLengthValue")]
    pub time_length_value: i64,
}

#[derive(Debug, Deserialize)]
pub struct SpecValue {
    #[serde(default, rename = "specValueList")]
    pub spec_value_list: Option<Vec<SpecValueResponse>>,
}

#[derive(Debug, Deserialize)]
pub struct PddServicemarketContractSearchResponse {
    /// 授权结束时间
    #[serde(default, rename = "endAt")]
    pub end_at: i64,
    /// 订单号列表
    #[serde(default, rename = "orderSns")]
    pub order_sns: Vec<String>,
    #[serde(default, rename = "specValue")]
    pub spec_value: Option<SpecValue>,
    /// 授权开始时间
    #[serde(default, rename = "startAt")]
    pub start_at: i64,
}
