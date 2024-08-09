//! 更新全站推广名称

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrUpdateAdNameRequest {
    /// 广告名称
    #[serde(rename = "adName")]
    pub ad_name: String,
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
}

impl RequestType for PddAdApiUnitTrUpdateAdNameRequest {
    type Response = PddAdApiUnitTrUpdateAdNameResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.update.ad.name"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrUpdateAdNameResponse {
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
