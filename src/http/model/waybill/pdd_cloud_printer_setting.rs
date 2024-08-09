//! # 云打印机设置
//!
//! 设置云打印机
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 打印浓度（1-淡，2-正常，3-浓）
    pub density: i32,
    /// 打印机标识
    pub printer_id: String,
    /// 共享码
    pub share_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddCloudPrinterSettingRequest {
    /// 开平请求基类
    pub request: Request,
}

impl RequestType for PddCloudPrinterSettingRequest {
    type Response = PddCloudPrinterSettingResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloud.printer.setting"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 打印机设置修改结果
    #[serde(default)]
    pub cloud_printer_setting_result: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudPrinterSettingResponse {
    #[serde(default)]
    pub error_code: i32,
    #[serde(default)]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
