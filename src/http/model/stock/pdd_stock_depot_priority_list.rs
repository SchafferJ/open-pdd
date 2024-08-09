//! # 仓库优先级列表
//!
//! 获取仓库优先级列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddStockDepotPriorityListRequest {
    /// 省id
    pub province_id: Option<i32>,
    /// 市id
    pub city_id: Option<i32>,
    /// 区id
    pub district_id: Option<i32>,
    /// 仓库编码
    pub depot_code: Option<String>,
    /// 每页数据显示数量
    pub page_size: i32,
    /// 当前页数 从1开始
    pub page_num: i32,
}

impl RequestType for PddStockDepotPriorityListRequest {
    type Response = PddStockDepotPriorityListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.depot.priority.list"
    }
}

#[derive(Debug, Deserialize)]
pub struct Depot {
    /// 仓库id
    #[serde(default)]
    pub depot_id: String,
    /// 仓库地址（省编号）
    #[serde(default)]
    pub province_id: i32,
    /// 仓库地址（市编号）
    #[serde(default)]
    pub city_id: i32,
    /// 仓库地址（区编号）
    #[serde(default)]
    pub district_id: i32,
    /// 仓库编码
    #[serde(default)]
    pub depot_code: String,
    /// 仓库名称
    #[serde(default)]
    pub depot_name: String,
    /// 优先级
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddStockDepotPriorityListResponse {
    /// 总数
    #[serde(default)]
    pub count: i32,
    /// 仓库列表
    #[serde(default)]
    pub depot_list: Option<Vec<Depot>>,
}
