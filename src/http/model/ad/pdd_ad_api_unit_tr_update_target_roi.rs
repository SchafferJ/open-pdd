//! 更新全站推广目标投产比

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrUpdateTargetRoiRequest {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// 目标roi 范围1000~1000000(万分位，表示0.1~100)
    #[serde(rename = "targetRoi")]
    pub target_roi: i64,
}

impl RequestType for PddAdApiUnitTrUpdateTargetRoiRequest {
    type Response = PddAdApiUnitTrUpdateTargetRoiResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.update.target.roi"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrUpdateTargetRoiResponse {
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
