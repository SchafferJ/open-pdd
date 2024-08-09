//! 批量启动或暂停创意

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreativeUpdateDataOperateStatusRequest {
    /// 数据操作状态。1表示开启，2表示暂停。
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: i32,
    /// 创意单元Id列表
    #[serde(rename = "unitCreativeIds")]
    pub unit_creative_ids: Vec<i64>,
}

impl RequestType for PddAdApiUnitCreativeUpdateDataOperateStatusRequest {
    type Response = PddAdApiUnitCreativeUpdateDataOperateStatusResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.creative.update.data.operate.status"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreativeUpdateDataOperateStatusResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否更改成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
