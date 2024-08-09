//! 获取定向标签数据

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitBidQueryTargetingTagListRequest;

impl RequestType for PddAdApiUnitBidQueryTargetingTagListRequest {
    type Response = PddAdApiUnitBidQueryTargetingTagListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.bid.query.targeting.tag.list"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 标签描述
    #[serde(default)]
    pub description: String,
    /// 父标签Id
    #[serde(default, rename = "parentTagId")]
    pub parent_tag_id: String,
    /// 标签Id
    #[serde(default, rename = "tagId")]
    pub tag_id: String,
    /// 定向标签类型，1-地域定向
    #[serde(default, rename = "tagType")]
    pub tag_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitBidQueryTargetingTagListResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
