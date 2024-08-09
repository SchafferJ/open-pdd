//! # 售后校验接口
//!
//! 校验售后单
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddRefundStatusCheckRequest {
    /// 20150909-452750051,20150909-452750134 用逗号分开
    pub order_sns: String,
}

impl RequestType for PddRefundStatusCheckRequest {
    type Response = PddRefundStatusCheckResponse;

    fn get_type(&self) -> &'static str {
        "pdd.refund.status.check"
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderSnsExistsRefund {
    /// 订单编号
    #[serde(default)]
    pub order_sn: String,
}

#[derive(Debug, Deserialize)]
pub struct PddRefundStatusCheckResponse {
    /// 存在售后的订单
    #[serde(default)]
    pub order_sns_exists_refund: Option<Vec<OrderSnsExistsRefund>>,
}
