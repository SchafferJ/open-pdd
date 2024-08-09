//! # 删除货品
//!
//! 家电分仓库存-删除货品
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddStockWareDeleteRequest {
    /// 货品id
    pub ware_id: i64,
}

impl RequestType for PddStockWareDeleteRequest {
    type Response = PddStockWareDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.delete"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareDeleteResponse;
