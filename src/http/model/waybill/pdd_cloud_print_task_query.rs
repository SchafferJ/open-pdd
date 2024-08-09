//! # 云打印任务查询
//!
//! 云打印任务查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CloudPrintTaskQuery {
    /// 打印序号，非必填，填了则只查询列表内的任务
    pub print_sequence_list: Option<Vec<i32>>,
    /// 打印任务id
    pub print_task_id: String,
    /// 打印机id
    pub printer_id: String,
    /// 共享码
    pub share_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddCloudPrintTaskQueryRequest {
    /// 云打印任务查询请求
    pub cloud_print_task_query: CloudPrintTaskQuery,
}

impl RequestType for PddCloudPrintTaskQueryRequest {
    type Response = PddCloudPrintTaskQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloud.print.task.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct CloudPrintTaskResult {
    /// 失败原因
    #[serde(default)]
    pub fail_reason: String,
    /// 请求云打印时token对应的mallId
    #[serde(default)]
    pub mall_id: i64,
    /// 打印序号
    #[serde(default)]
    pub print_sequence: i32,
    /// 打印状态 0:打印中 1:打印成功 2:打印失败 3:打印超时
    #[serde(default)]
    pub print_status: i32,
    /// 打印任务id
    #[serde(default)]
    pub print_task_id: String,
    /// 是否推送给云打印机，只有打印失败的任务才有值，如果为true，需要重点关注是否打印完成
    #[serde(default)]
    pub pushed_to_printer: bool,
    /// 快递公司编码
    #[serde(default)]
    pub ship_code: String,
    /// 运单号
    #[serde(default)]
    pub waybill_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 云打印任务结果列表
    #[serde(default)]
    pub cloud_print_task_result_list: Option<Vec<CloudPrintTaskResult>>,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudPrintTaskQueryResponse {
    #[serde(default)]
    pub error_code: i32,
    #[serde(default)]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
