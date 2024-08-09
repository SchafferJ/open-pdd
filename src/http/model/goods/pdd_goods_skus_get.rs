//! # 根据skuId查询sku详情
//!
//! 库存同步、改价等场景
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSkusGetRequest {
    /// 商品Id
    pub goods_id: i64,
    /// sku id
    pub sku_id: i64,
}

impl RequestType for PddGoodsSkusGetRequest {
    type Response = PddGoodsSkusGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.skus.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    /// 规格项ID
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 规格项
    #[serde(default)]
    pub parent_spec_name: String,
    /// 规格ID
    #[serde(default)]
    pub spec_id: i64,
    /// 规格值
    #[serde(default)]
    pub spec_name: String,
    /// 规格备注
    #[serde(default)]
    pub spec_note: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSkusGetResponse {
    /// 该sku所在的商品ID
    #[serde(default)]
    pub goods_id: i64,
    /// 上下架状态，1=上架，0=下架
    #[serde(default)]
    pub is_onsale: i32,
    /// 团购价，单位分
    #[serde(default)]
    pub multi_price: i64,
    /// SKU维度商家外部编码
    #[serde(default)]
    pub out_sku_sn: String,
    /// 库存
    #[serde(default)]
    pub quantity: i64,
    /// 预扣库存
    #[serde(default)]
    pub reserve_quantity: i64,
    /// 单买价，单位分
    #[serde(default)]
    pub single_price: i64,
    /// skuId
    #[serde(default)]
    pub sku_id: i64,
    /// 规格列表
    #[serde(default)]
    pub spec: Option<Vec<Spec>>,
}
