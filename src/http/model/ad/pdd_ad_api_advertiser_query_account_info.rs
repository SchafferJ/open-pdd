//! 查询广告开户信息

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiAdvertiserQueryAccountInfoRequest;

impl RequestType for PddAdApiAdvertiserQueryAccountInfoRequest {
    type Response = PddAdApiAdvertiserQueryAccountInfoResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.advertiser.query.account.info"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 是否已开户
    #[serde(default, rename = "hasOpenAccount")]
    pub has_open_account: bool,
    /// 是否已缴纳保证金
    #[serde(default, rename = "hasPaidDeposit")]
    pub has_paid_deposit: bool,
    /// 是否已签署广告协议
    #[serde(default, rename = "hasSignProtocol")]
    pub has_sign_protocol: bool,
    /// 广告主ID
    #[serde(default, rename = "mallId")]
    pub mall_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiAdvertiserQueryAccountInfoResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
