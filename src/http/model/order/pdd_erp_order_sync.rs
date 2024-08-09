//! # erp打单信息同步
//!
//! erp打单信息同步
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddErpOrderSyncRequest {
    /// 物流公司编码
    pub logistics_id: i64,
    /// 订单号
    pub order_sn: String,
    /// 订单状态：1-已打单
    pub order_state: i32,
    /// 运单号
    pub waybill_no: String,
}

impl RequestType for PddErpOrderSyncRequest {
    type Response = PddErpOrderSyncResponse;

    fn get_type(&self) -> &'static str {
        "pdd.erp.order.sync"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddErpOrderSyncResponse;
