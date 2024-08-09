//! 批量修改关键词

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Keywords {
    /// 关键词出价
    pub bid: i64,
    /// 关键词Id
    #[serde(rename = "keywordId")]
    pub keyword_id: i64,
    /// 关键词溢价比例。万分比。
    #[serde(rename = "premiumRate")]
    pub premium_rate: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiKeywordUpdateRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 关键词列表
    pub keywords: Vec<Keywords>,
}

impl RequestType for PddAdApiKeywordUpdateRequest {
    type Response = PddAdApiKeywordUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.keyword.update"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiKeywordUpdateResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否更新成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
