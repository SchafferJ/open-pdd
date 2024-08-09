//! # 旅游门票订单售后结果回调
//!
//! 供应商向拼多多回调售后就结果
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddTicketOrderRefundNotifycationRequest {
    /// 拼多多制票号
    pub order_no: String,
    /// 退款金额（分） status=2时必传
    pub refund_amount: Option<i64>,
    /// 驳回原因 status=3时必传
    pub reject_reason: Option<String>,
    /// 退款流水号
    pub serial_no: String,
    /// 受理状态。2.已通过 3.已驳回
    pub status: i32,
}

impl RequestType for PddTicketOrderRefundNotifycationRequest {
    type Response = PddTicketOrderRefundNotifycationResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.order.refund.notifycation"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTicketOrderRefundNotifycationResponse {
    #[serde(default)]
    pub success: bool,
}
