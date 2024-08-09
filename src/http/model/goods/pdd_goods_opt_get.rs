//! # 查询商品标签列表
//!
//! 获得拼多多商品标签列表（非商品类目cat，当前仅开放给多多客使用）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsOptGetRequest {
    /// 值=0时为顶点opt_id,通过树顶级节点获取opt树
    pub parent_opt_id: i32,
}

impl RequestType for PddGoodsOptGetRequest {
    type Response = PddGoodsOptGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.opt.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct GoodsOpt {
    /// 层级，1-一级，2-二级，3-三级，4-四级
    #[serde(default)]
    pub level: i32,
    /// 商品标签ID
    #[serde(default)]
    pub opt_id: i64,
    /// 商品标签名
    #[serde(default)]
    pub opt_name: String,
    /// id所属父ID，其中，parent_id=0时为顶级节点
    #[serde(default)]
    pub parent_opt_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsOptGetResponse {
    /// opt列表
    #[serde(default)]
    pub goods_opt_list: Option<Vec<GoodsOpt>>,
}
