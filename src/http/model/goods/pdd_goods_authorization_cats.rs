//! # 获取当前授权商家可发布的商品类目信息
//!
//! 获取当前授权商家可发布的商品类目信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsAuthorizationCatsRequest {
    /// 默认值=0，值=0时为顶点cat_id,通过树顶级节点获取一级类目
    pub parent_cat_id: Option<i64>,
}

impl RequestType for PddGoodsAuthorizationCatsRequest {
    type Response = PddGoodsAuthorizationCatsResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.authorization.cats"
    }
}

#[derive(Debug, Deserialize)]
pub struct GoodsCats {
    /// 类目ID，一级类目ID
    #[serde(default)]
    pub cat_id: i64,
    /// 对应ID下的类目名称
    #[serde(default)]
    pub cat_name: String,
    /// 是否为叶子类目
    #[serde(default)]
    pub leaf: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsAuthorizationCatsResponse {
    /// 类目对象列表
    #[serde(default)]
    pub goods_cats_list: Option<Vec<GoodsCats>>,
}
