//! # 商家售后同意退货
//!
//! 开放平台商家同意退货
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 售后id
    pub after_sales_id: i64,
    /// 给用户留言
    pub operate_desc: String,
    /// 订单号
    pub order_sn: String,
    /// 退货地址列表中已有的退货地址id
    pub return_address_id: String,
}

#[derive(Debug, Serialize)]
pub struct PddRefundReturngoodsAgreeRequest {
    /// 请求入参
    pub request: Request,
}

impl RequestType for PddRefundReturngoodsAgreeRequest {
    type Response = PddRefundReturngoodsAgreeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.refund.returngoods.agree"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 售后id
    #[serde(default)]
    pub after_sales_id: i64,
    /// 退货是否成功描述
    #[serde(default)]
    pub message: String,
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
    /// 退货是否成功
    #[serde(default)]
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddRefundReturngoodsAgreeResponse {
    /// 接口调用错误编码
    #[serde(default)]
    pub error_code: i32,
    /// 接口调用错误描述
    #[serde(default)]
    pub error_msg: String,
    /// 返回内容
    #[serde(default)]
    pub result: Option<Result>,
    /// 接口是否调用成功
    #[serde(default)]
    pub success: bool,
}
