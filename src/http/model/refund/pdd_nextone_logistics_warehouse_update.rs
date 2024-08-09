//! # 退货入库
//!
//! 退货入库通知拼多多确认入库成功
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 售后id
    pub after_sales_id: i64,
    /// 操作时间
    pub operate_time: i64,
    /// order_sn
    pub order_sn: String,
    /// 物流公司id
    pub reverse_logistics_id: Option<i32>,
    /// 物流单号
    pub reverse_tracking_number: String,
    /// 退货入库状态 1：成功；2：失败
    pub warehouse_status: i32,
}

#[derive(Debug, Serialize)]
pub struct PddNextoneLogisticsWarehouseUpdateRequest {
    /// request
    pub request: Request,
}

impl RequestType for PddNextoneLogisticsWarehouseUpdateRequest {
    type Response = PddNextoneLogisticsWarehouseUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.nextone.logistics.warehouse.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// after_sales_id
    #[serde(default)]
    pub after_sales_id: i64,
    /// order_sn
    #[serde(default)]
    pub order_sn: String,
}

#[derive(Debug, Deserialize)]
pub struct PddNextoneLogisticsWarehouseUpdateResponse {
    /// result
    #[serde(default)]
    pub result: Option<Result>,
}
