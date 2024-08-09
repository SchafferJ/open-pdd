//! # 标品详情接口
//!
//! 根据标品类目和关键属性获取标品详情信息，可以先通过pdd.goods.spu.search接口获取标品的类目及关键属性。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct KeyProp {
    /// 引用属性ID
    pub ref_pid: Option<i64>,
    /// 属性值单位
    pub value_unit: Option<String>,
    /// 关键属性值，和vid必须入参其一。
    pub value: Option<String>,
    /// 关键属性值ID，和value必须入参其一。
    pub vid: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsSpuGetRequest {
    /// 标品所在的类目ID
    pub cat_id: String,
    /// 关键属性
    pub key_prop: Vec<KeyProp>,
}

impl RequestType for PddGoodsSpuGetRequest {
    type Response = PddGoodsSpuGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.spu.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct BindProp {
    /// 属性名
    #[serde(default)]
    pub pname: String,
    /// 引用属性ID
    #[serde(default)]
    pub ref_pid: i64,
    /// 属性值单位
    #[serde(default)]
    pub value_unit: String,
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 属性值ID
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct KeyPropResponse {
    /// 属性名
    #[serde(default)]
    pub pname: String,
    /// 引用属性id
    #[serde(default)]
    pub ref_pid: i64,
    /// 属性值单位
    #[serde(default)]
    pub value_unit: String,
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 属性值ID
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct SaleProp {
    /// 组ID
    #[serde(default)]
    pub group_id: i32,
    /// 父规格ID
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 属性名
    #[serde(default)]
    pub pname: String,
    /// 引用属性ID
    #[serde(default)]
    pub ref_pid: i64,
    /// 规格ID
    #[serde(default)]
    pub spec_id: i64,
    /// 属性值单位
    #[serde(default)]
    pub value_unit: String,
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 属性值ID
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSpuGetResponse {
    /// 绑定属性，商品属性中对应的引用属性的属性值需要和绑定属性值相同。
    #[serde(default)]
    pub bind_prop: Option<Vec<BindProp>>,
    /// 商品轮播图
    #[serde(default)]
    pub carousel_gallery: Vec<String>,
    /// 标品所在的类目ID。若非叶子类目，表示该标品可用于该类目下的任何叶子类目。
    #[serde(default)]
    pub cat_id: i64,
    /// 商品详情图
    #[serde(default)]
    pub detail_gallery: Vec<String>,
    /// 商品标题
    #[serde(default)]
    pub goods_name: String,
    /// 关键属性
    #[serde(default)]
    pub key_prop: Option<Vec<KeyPropResponse>>,
    /// 商品前缀标题，若有返回值，则表示发布该标品对应的商品时，商品标题的开头需要包含这部分字符串。
    #[serde(default)]
    pub pre_title: String,
    /// 销售属性，商品属性中对应的引用属性的属性值需要是销售属性值的子集。
    #[serde(default)]
    pub sale_prop: Option<Vec<SaleProp>>,
    /// 标品标题
    #[serde(default)]
    pub spu_name: String,
}
