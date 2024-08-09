//! # 旅游门票订单核销通知接口
//!
//! 供应商向拼多多发送订单核销通知
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddTicketVerificationNotifycationRequest {
    /// 拼多多制票号
    pub order_no: String,
    /// 核销时间（13位毫秒数）
    pub verify_time: i64,
}

impl RequestType for PddTicketVerificationNotifycationRequest {
    type Response = PddTicketVerificationNotifycationResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.verification.notifycation"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTicketVerificationNotifycationResponse {
    #[serde(default)]
    pub success: bool,
}
