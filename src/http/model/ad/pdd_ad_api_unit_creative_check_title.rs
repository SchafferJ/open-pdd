//! 检查创意标题是否合法

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreativeCheckTitleRequest {
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// 创意标题
    pub title: String,
}

impl RequestType for PddAdApiUnitCreativeCheckTitleRequest {
    type Response = PddAdApiUnitCreativeCheckTitleResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.creative.check.title"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreativeCheckTitleResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否合法
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
