//! 批量启停全站推广广告

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrUpdateDataOperateStatusRequest {
    /// 商家操作状态：1-启动 2-暂停
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: i32,
    /// 商品id列表
    #[serde(rename = "goodsIds")]
    pub goods_ids: Vec<i64>,
}

impl RequestType for PddAdApiUnitTrUpdateDataOperateStatusRequest {
    type Response = PddAdApiUnitTrUpdateDataOperateStatusResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.update.data.operate.status"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrUpdateDataOperateStatusResponse {
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
