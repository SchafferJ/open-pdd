//! # 标品搜索接口
//!
//! 可以通过标品名称或者类目+关键属性的值两种模式进行搜索，搜索到的标品需要在标品详情接口获取详细信息。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct KeyProp {
    /// 关键属性的引用属性ID，需要从pdd.cat.rule.get中获取。
    pub ref_pid: Option<i64>,
    /// 关键属性值，需要从pdd.goods.cat.rule.get中获取。当要根据关键属性匹配时，和vid必须入参其一。
    pub value: Option<String>,
    /// 属性值单位
    pub value_unit: Option<String>,
    /// 关键属性值ID，需要从pdd.goods.cat.rule.get中获取规则。当要根据关键属性匹配时，和value必须入参其一。
    pub vid: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsSpuSearchRequest {
    /// 类目ID，可以是一二三四级类目，在该类目下进行搜索。
    pub cat_id: i64,
    /// 标品关键属性精确匹配。和标品标题必须入参其一。
    pub key_prop: Option<Vec<KeyProp>>,
    /// 标品标题模糊搜索。和关键属性必须入参其一。
    pub spu_name: Option<String>,
}

impl RequestType for PddGoodsSpuSearchRequest {
    type Response = PddGoodsSpuSearchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.spu.search"
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyPropResponse {
    /// 引用属性ID
    #[serde(default)]
    pub ref_pid: i64,
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 属性值单位
    #[serde(default)]
    pub value_unit: String,
    /// 属性值ID
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct Spu {
    /// 标品所在的类目ID。若非叶子类目，表示该标品可用于该类目下的任何叶子类目。
    #[serde(default)]
    pub cat_id: i64,
    /// 关键属性
    #[serde(default)]
    pub key_prop: Option<Vec<KeyPropResponse>>,
    /// 标品标题
    #[serde(default)]
    pub spu_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSpuSearchResponse {
    /// 标品列表
    #[serde(default)]
    pub spu_list: Option<Vec<Spu>>,
}
