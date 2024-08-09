//! 批量创建关键词

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Keyword {
    /// 关键词出价
    pub bid: i64,
    /// 关键词溢价比例。万分比。
    #[serde(rename = "premiumRate")]
    pub premium_rate: Option<i64>,
    /// 关键词
    pub word: String,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiKeywordCreateRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 关键词创建信息列表
    #[serde(rename = "keywordList")]
    pub keyword_list: Vec<Keyword>,
}

impl RequestType for PddAdApiKeywordCreateRequest {
    type Response = PddAdApiKeywordCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.keyword.create"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiKeywordCreateResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否创建成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
