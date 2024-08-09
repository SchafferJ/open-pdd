//! # 根据商品id查询sku信息
//!
//! 家电分仓库存-根据商品id查询sku信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddStockGoodsIdToSkuQueryRequest {
    /// 商品id
    pub goods_id: i64,
    /// 是否需要查询下架商品和sku，默认不需要
    pub need_offsale: Option<bool>,
    /// 货品id
    pub ware_id: Option<i64>,
}

impl RequestType for PddStockGoodsIdToSkuQueryRequest {
    type Response = PddStockGoodsIdToSkuQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.goods.id.to.sku.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct Specs {
    /// 规格id
    #[serde(default)]
    pub spec_id: i64,
    /// 规格名称
    #[serde(default)]
    pub spec_key: String,
    /// 规格值
    #[serde(default)]
    pub spec_value: String,
}

#[derive(Debug, Deserialize)]
pub struct Skus {
    /// 是否已经绑定货品false/true
    #[serde(default)]
    pub exist_ware: bool,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 上下架状态，true表示上架，false表示下架
    #[serde(default)]
    pub is_onsale: bool,
    /// sku_id
    #[serde(default)]
    pub sku_id: i64,
    /// 规格信息
    #[serde(default)]
    pub specs: Option<Vec<Specs>>,
    /// 货品id
    #[serde(default)]
    pub ware_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddStockGoodsIdToSkuQueryResponse {
    /// sku信息
    #[serde(default)]
    pub skus: Option<Vec<Skus>>,
    /// 总数
    #[serde(default)]
    pub total: i32,
}
