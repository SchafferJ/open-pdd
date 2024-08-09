//! 更新全站推广日预算

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrUpdateMaxCostRequest {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// 单日消耗上线（厘），范围100000~1000000000（表示100-1000000元）
    #[serde(rename = "maxCost")]
    pub max_cost: i64,
}

impl RequestType for PddAdApiUnitTrUpdateMaxCostRequest {
    type Response = PddAdApiUnitTrUpdateMaxCostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.update.max.cost"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrUpdateMaxCostResponse {
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
