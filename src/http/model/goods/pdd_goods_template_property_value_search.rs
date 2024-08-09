//! # 模板属性值搜索接口
//!
//! 商品发布过程中，填写商品属性时，用于模糊搜索属性模板上可填属性值
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsTemplatePropertyValueSearchRequest {
    /// 类目id
    pub cat_id: i64,
    /// 页码 从1开始
    pub page_num: Option<i32>,
    /// 1页查询的最大数据 [1，500], 默认100
    pub page_size: Option<i32>,
    /// 父属性值id
    pub parent_vid: Option<i64>,
    /// 模板属性id，废弃中，请入参属性id
    pub template_pid: Option<i64>,
    /// 需要模糊搜索的属性值
    pub value: String,
    /// 属性id
    pub ref_pid: Option<i64>,
}

impl RequestType for PddGoodsTemplatePropertyValueSearchRequest {
    type Response = PddGoodsTemplatePropertyValueSearchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.template.property.value.search"
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 属性值id
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsTemplatePropertyValueSearchResponse {
    /// list
    #[serde(default)]
    pub list: Option<Vec<List>>,
}
