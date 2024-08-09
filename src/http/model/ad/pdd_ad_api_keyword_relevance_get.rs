//! 关键词相关性查询

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiKeywordRelevanceGetRequest {
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// 关键词数组
    pub words: Vec<String>,
}

impl RequestType for PddAdApiKeywordRelevanceGetRequest {
    type Response = PddAdApiKeywordRelevanceGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.keyword.relevance.get"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 相关性指标，5最高，值越大相关度越高
    #[serde(default)]
    pub relevance: i32,
    /// 关键词
    #[serde(default)]
    pub word: String,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiKeywordRelevanceGetResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
