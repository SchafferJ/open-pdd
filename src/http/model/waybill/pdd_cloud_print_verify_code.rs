//! # 云打印验证码
//!
//! 云打印机绑定时，打印验证码
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CloudPrintVerifyCodeRequest {
    /// 打印机id
    pub printer_id: String,
}

#[derive(Debug, Serialize)]
pub struct PddCloudPrintVerifyCodeRequest {
    /// 云打印验证码请求
    pub cloud_print_verify_code_request: CloudPrintVerifyCodeRequest,
}

impl RequestType for PddCloudPrintVerifyCodeRequest {
    type Response = PddCloudPrintVerifyCodeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloud.print.verify.code"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// true:请求验证码成功 false:请求验证码失败
    #[serde(default)]
    pub result: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudPrintVerifyCodeResponse {
    /// 错误码
    #[serde(default)]
    pub error_code: i32,
    /// 错误信息
    #[serde(default)]
    pub error_msg: String,
    /// 响应结果
    #[serde(default)]
    pub result: Option<Result>,
    /// 是否成功
    #[serde(default)]
    pub success: bool,
}
