//! # 类目预测接口
//!
//! 类目预测
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsOuterCatMappingGetRequest {
    /// 外部叶子类目id
    pub outer_cat_id: i64,
    /// 外部叶子类目名称
    pub outer_cat_name: String,
    /// 外部商品名称
    pub outer_goods_name: String,
}

impl RequestType for PddGoodsOuterCatMappingGetRequest {
    type Response = PddGoodsOuterCatMappingGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.outer.cat.mapping.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsOuterCatMappingGetResponse {
    /// 一级类目
    #[serde(default)]
    pub cat_id1: i64,
    /// 二级类目
    #[serde(default)]
    pub cat_id2: i64,
    /// 三级类目
    #[serde(default)]
    pub cat_id3: i64,
    /// 四级类目
    #[serde(default)]
    pub cat_id4: i64,
}
