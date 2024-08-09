//! 批量启动或暂停单元

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitUpdateDataOperateStatusRequest {
    /// 广告单元Id列表。一次不得超过20个。
    #[serde(rename = "adIds")]
    pub ad_ids: Vec<i64>,
    /// 数据操作状态。1表示开启，2表示暂停。
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: i32,
}

impl RequestType for PddAdApiUnitUpdateDataOperateStatusRequest {
    type Response = PddAdApiUnitUpdateDataOperateStatusResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.update.data.operate.status"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitUpdateDataOperateStatusResponse {
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
