//! # 商品属性类目接口
//!
//! 获取商品规格信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSpecGetRequest {
    /// 叶子类目ID，必须入参level=3时的cat_id,否则无法返回正确的参数
    pub cat_id: i64,
}

impl RequestType for PddGoodsSpecGetRequest {
    type Response = PddGoodsSpecGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.spec.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct GoodsSpec {
    /// 规格所属的叶子类目ID
    #[serde(default)]
    pub cat_id: i64,
    /// 商品规格对应的ID
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 商品规格ID对应的规格名称
    #[serde(default)]
    pub parent_spec_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSpecGetResponse {
    /// 规格列表对象
    #[serde(default)]
    pub goods_spec_list: Option<Vec<GoodsSpec>>,
}
