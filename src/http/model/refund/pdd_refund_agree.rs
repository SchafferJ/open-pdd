//! # 同意退款
//!
//! 商家同意退款
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 售后id
    pub after_sales_id: i64,
    /// 退款备注，商家留言
    pub operate_desc: Option<String>,
    /// 订单编号
    pub order_sn: String,
}

#[derive(Debug, Serialize)]
pub struct PddRefundAgreeRequest {
    /// request
    pub request: Request,
}

impl RequestType for PddRefundAgreeRequest {
    type Response = PddRefundAgreeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.refund.agree"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    #[serde(default)]
    pub after_sales_id: i64,
    /// 退款操作成功      退款操作结果信息。如果退款失败，会返回失败原因
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub order_sn: String,
    /// 接口调用成功
    #[serde(default)]
    pub succ: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddRefundAgreeResponse {
    #[serde(default)]
    pub result: Option<Result>,
}
