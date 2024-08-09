//! # 货品列表的查询接口
//!
//! 获取货品列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddStockWareInfoListRequest {
    /// 货品编码集合
    pub ware_sn_list: Vec<String>,
}

impl RequestType for PddStockWareInfoListRequest {
    type Response = PddStockWareInfoListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.info.list"
    }
}

#[derive(Debug, Deserialize)]
pub struct Ware {
    /// 货品编码
    #[serde(default)]
    pub ware_sn: String,
    /// 货品ID
    #[serde(default)]
    pub ware_id: i64,
    /// 是否删除
    #[serde(default)]
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareInfoListResponse {
    /// 货品信息列表
    #[serde(default)]
    pub ware_list: Option<Vec<Ware>>,
}
