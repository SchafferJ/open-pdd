//! # 批量更新仓库优先级
//!
//! 批量更新仓库优先级
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Priority {
    /// 优先级 范围是1 - 999
    pub priority: i32,
    /// 区id
    pub district_id: i32,
    /// 市id
    pub city_id: i32,
    /// 省id
    pub province_id: i32,
    /// 仓库id
    pub depot_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PddStockDepotPriorityUpdateRequest {
    /// 示例：[{"depot_id":1,"province_id":12,"city_id":34,"district_id":56,"priority":5}]
    pub priority_list: Vec<Priority>,
}

impl RequestType for PddStockDepotPriorityUpdateRequest {
    type Response = PddStockDepotPriorityUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.depot.priority.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddStockDepotPriorityUpdateResponse;
