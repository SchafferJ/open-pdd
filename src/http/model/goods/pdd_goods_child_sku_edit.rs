//! # 日历库存子SKU新增及编辑接口
//!
//! 日历库存类商品编辑或新增价格日历接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ChildSkus {
    /// 售卖日期（“yyyy-MM-dd”）
    pub date: String,
    /// 团购价
    pub group_price: i64,
    /// 库存增减
    pub quantity_delta: i64,
    /// 单买价
    pub single_price: i64,
}

#[derive(Debug, Serialize)]
pub struct Skus {
    /// 日历库存商品子sku信息列表
    pub child_skus: Vec<ChildSkus>,
    /// 上架状态。0=已下架，1=已上架。不传表示不做修改
    pub is_onsale: Option<i32>,
    /// 与sku_id必填其一，用于确定编辑的sku，当有多个sku的out_sku_sn一样时会编辑失败。
    pub out_sku_sn: Option<String>,
    /// 日历库存商品父skuId
    pub sku_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsChildSkuEditRequest {
    /// 草稿id（未填写则新建一条商品草稿）
    pub goods_commit_id: Option<i64>,
    /// 商品id
    pub goods_id: i64,
    /// 日历库存型商品sku信息列表
    pub skus: Vec<Skus>,
    /// 提交后上下架状态，0=上架；1=保持原样。表示编辑商品并提交后商品的上下架状态，不传时默认为0，上架。
    pub sync_goods_operate: Option<i32>,
}

impl RequestType for PddGoodsChildSkuEditRequest {
    type Response = PddGoodsChildSkuEditResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.child.sku.edit"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsChildSkuEditResponse {
    /// 草稿id
    #[serde(default)]
    pub goods_commit_id: i64,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
}
