//! # 货品关联sku
//!
//! 家电分仓库存-货品关联sku
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct WareSkus {
    /// sku id
    pub sku_id: i64,
    /// 商品id
    pub goods_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PddStockWareSkuUpdateRequest {
    /// 货品id
    pub ware_id: i64,
    /// 组合货品中子货品的关联关系
    pub ware_skus: Vec<WareSkus>,
}

impl RequestType for PddStockWareSkuUpdateRequest {
    type Response = PddStockWareSkuUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.sku.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareSkuUpdateResponse;
