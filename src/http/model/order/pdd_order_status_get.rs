//! # 订单状态
//!
//! 获取订单的状态
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderStatusGetRequest {
    /// 20150909-452750051,20150909-452750134 用逗号分开
    pub order_sns: String,
}

impl RequestType for PddOrderStatusGetRequest {
    type Response = PddOrderStatusGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.status.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderStatus {
    /// 订单编号
    #[serde(default, rename = "orderSn")]
    pub order_sn: String,
    /// 订单发货状态，1：待发货，2：已发货待签收，3：已签收 0：异常
    #[serde(default)]
    pub order_status: i32,
    /// 订单售后状态，1：无售后或售后关闭，2：售后处理中，3：退款中，4：退款成功，0：异常
    #[serde(default)]
    pub refund_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddOrderStatusGetResponse {
    /// 订单状态列表对象
    #[serde(default)]
    pub order_status_list: Option<Vec<OrderStatus>>,
}
