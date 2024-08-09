//! # 商品地区/国家接口
//!
//! 获取指定国家或地区信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCountryGetRequest;

impl RequestType for PddGoodsCountryGetRequest {
    type Response = PddGoodsCountryGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.country.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Country {
    /// 国家或地区对应的ID
    #[serde(default)]
    pub country_id: i64,
    /// 对应ID下的国家或地区名称
    #[serde(default)]
    pub country_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCountryGetResponse {
    /// 国家或地区对象列表
    #[serde(default)]
    pub country_list: Option<Vec<Country>>,
}
