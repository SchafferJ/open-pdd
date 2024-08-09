//! # 云打印机状态查询
//!
//! 查询云打印机状态
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CloudPrinterStatusQueryRequest {
    /// 打印机id
    pub printer_id: String,
    /// 共享码
    pub share_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddCloudPrinterStatusQueryRequest {
    /// 打印状态查询请求
    pub cloud_printer_status_query_request: CloudPrinterStatusQueryRequest,
}

impl RequestType for PddCloudPrinterStatusQueryRequest {
    type Response = PddCloudPrinterStatusQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloud.printer.status.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 打印机id
    #[serde(default)]
    pub printer_id: String,
    /// 信号强度dbm
    #[serde(default)]
    pub signal_strength: String,
    /// 信号格度0-5,未查询到为null
    #[serde(default)]
    pub signal_strength_friendly: i32,
    /// 打印机状态 -1:未知  0:未找到打印机  1:不在线  2:在线
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudPrinterStatusQueryResponse {
    #[serde(default)]
    pub error_code: i32,
    #[serde(default)]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
