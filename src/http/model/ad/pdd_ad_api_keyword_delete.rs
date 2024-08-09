//! 批量删除关键词

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiKeywordDeleteRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 关键词Id列表
    #[serde(rename = "keywordIds")]
    pub keyword_ids: Vec<i64>,
}

impl RequestType for PddAdApiKeywordDeleteRequest {
    type Response = PddAdApiKeywordDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.keyword.delete"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiKeywordDeleteResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否删除成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
