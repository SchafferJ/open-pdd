//! # 商品标准类目接口
//!
//! 获取拼多多标准商品类目信息（请使用pdd.goods.authorization.cats接口获取商家可发布类目）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCatsGetRequest {
    /// 值=0时为顶点cat_id,通过树顶级节点获取cat树
    pub parent_cat_id: i64,
}

impl RequestType for PddGoodsCatsGetRequest {
    type Response = PddGoodsCatsGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cats.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct GoodsCats {
    /// 商品类目ID
    #[serde(default)]
    pub cat_id: i64,
    /// 商品类目名称
    #[serde(default)]
    pub cat_name: String,
    /// 类目层级，1-一级类目，2-二级类目，3-三级类目，4-四级类目
    #[serde(default)]
    pub level: i32,
    /// id所属父类目ID，其中，parent_id=0时为顶级节点
    #[serde(default)]
    pub parent_cat_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCatsGetResponse {
    /// 类目树对象
    #[serde(default)]
    pub goods_cats_list: Option<Vec<GoodsCats>>,
}
