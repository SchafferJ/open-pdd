//! # 货品仓库库存信息查询
//!
//! 通过货品编码查询货品和库存信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 当前页数
    pub page: i32,
    /// 页显示数据条数
    pub page_size: i32,
    /// 货品编码列表
    pub ware_sn_list: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PddStockWareWarehouseQueryRequest {
    /// 请求对象
    pub request: Request,
}

impl RequestType for PddStockWareWarehouseQueryRequest {
    type Response = PddStockWareWarehouseQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.warehouse.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareWarehouseQueryResponse {
    /// 总数
    #[serde(default)]
    pub total: i64,
    /// 查询结果
    #[serde(default)]
    pub ware_sn_warehouse_info: std::collections::BTreeMap<String, serde_json::Value>,
}
