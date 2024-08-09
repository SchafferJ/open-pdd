//! 启动或暂停计划

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanUpdateDataOperateStatusRequest {
    /// 数据操作状态。1表示开启，2表示暂停。
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: i32,
    /// 广告计划Id列表
    #[serde(rename = "planIds")]
    pub plan_ids: Vec<i64>,
    /// 场景类型。0表示搜索，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiPlanUpdateDataOperateStatusRequest {
    type Response = PddAdApiPlanUpdateDataOperateStatusResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.update.data.operate.status"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanUpdateDataOperateStatusResponse {
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
