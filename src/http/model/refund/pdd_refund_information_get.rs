//! # 售后单详情接口
//!
//! 查询单个售后单详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddRefundInformationGetRequest {
    /// 售后单id
    pub after_sales_id: Option<i64>,
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddRefundInformationGetRequest {
    type Response = PddRefundInformationGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.refund.information.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddRefundInformationGetResponse;
