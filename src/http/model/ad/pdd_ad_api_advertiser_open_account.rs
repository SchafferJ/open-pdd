//! 广告主开户

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiAdvertiserOpenAccountRequest;

impl RequestType for PddAdApiAdvertiserOpenAccountRequest {
    type Response = PddAdApiAdvertiserOpenAccountResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.advertiser.open.account"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiAdvertiserOpenAccountResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 开户结果
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
