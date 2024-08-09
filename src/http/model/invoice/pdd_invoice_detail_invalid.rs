//! # 订单发票冲红
//!
//! 根据订单号冲红发票
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddInvoiceDetailInvalidRequest {
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddInvoiceDetailInvalidRequest {
    type Response = PddInvoiceDetailInvalidResponse;

    fn get_type(&self) -> &'static str {
        "pdd.invoice.detail.invalid"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 红冲是否成功
    #[serde(default)]
    pub result: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddInvoiceDetailInvalidResponse {
    /// errorCode
    #[serde(default)]
    pub error_code: i32,
    /// errorMsg
    #[serde(default)]
    pub error_msg: String,
    /// result
    #[serde(default)]
    pub result: Option<Result>,
    /// success
    #[serde(default)]
    pub success: bool,
}
