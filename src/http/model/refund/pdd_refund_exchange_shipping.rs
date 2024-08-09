//! # 商家换货发货
//!
//! 商家换货发货
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 售后id
    pub after_sales_id: i64,
    /// 订单编号
    pub order_sn: String,
    /// 物流公司id
    pub shipping_id: i32,
    /// 物流公司名称
    pub shipping_name: String,
    /// 换货物流单号
    pub tracking_number: String,
}

#[derive(Debug, Serialize)]
pub struct PddRefundExchangeShippingRequest {
    /// request
    pub request: Request,
}

impl RequestType for PddRefundExchangeShippingRequest {
    type Response = PddRefundExchangeShippingResponse;

    fn get_type(&self) -> &'static str {
        "pdd.refund.exchange.shipping"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    #[serde(default)]
    pub after_sales_id: i64,
    /// 操作结果信息。如果失败，会返回失败原因
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub order_sn: String,
    /// 是否操作成功
    #[serde(default)]
    pub succ: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddRefundExchangeShippingResponse {
    #[serde(default)]
    pub result: Option<Result>,
}
