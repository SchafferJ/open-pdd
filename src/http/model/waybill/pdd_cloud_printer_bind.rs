//! # 云打印机绑定
//!
//! 云打印机绑定
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CloudPrinterBindRequest {
    /// 打印机id
    pub printer_id: String,
    /// 验证码
    pub verify_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddCloudPrinterBindRequest {
    /// 云打印绑定请求
    pub cloud_printer_bind_request: CloudPrinterBindRequest,
}

impl RequestType for PddCloudPrinterBindRequest {
    type Response = PddCloudPrinterBindResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloud.printer.bind"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    #[serde(default)]
    pub result: bool,
    #[serde(default, rename = "shareCode")]
    pub share_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudPrinterBindResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
