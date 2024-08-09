//! 删除全站推广单元

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrDeleteRequest {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
}

impl RequestType for PddAdApiUnitTrDeleteRequest {
    type Response = PddAdApiUnitTrDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.delete"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrDeleteResponse {
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
