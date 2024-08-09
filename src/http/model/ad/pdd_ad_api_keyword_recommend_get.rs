//! 获取推荐关键词

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiKeywordRecommendGetRequest {
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
}

impl RequestType for PddAdApiKeywordRecommendGetRequest {
    type Response = PddAdApiKeywordRecommendGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.keyword.recommend.get"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 关键词平均出价
    #[serde(default, rename = "avgBid")]
    pub avg_bid: f64,
    /// 竞争力
    #[serde(default)]
    pub compete: f64,
    /// 热度
    #[serde(default)]
    pub heat: f64,
    /// 关键词相关性
    #[serde(default)]
    pub relevance: i32,
    /// 质量分
    #[serde(default)]
    pub score: f64,
    /// 关键词趋势
    #[serde(default)]
    pub trend: f64,
    /// 关键词
    #[serde(default)]
    pub word: String,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiKeywordRecommendGetResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
