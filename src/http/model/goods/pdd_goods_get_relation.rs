//! # 商品映射查询接口
//!
//! 商品映射查询接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsGetRelationRequest {
    /// 拼多多商品id
    pub pdd_goods_id: Vec<i64>,
}

impl RequestType for PddGoodsGetRelationRequest {
    type Response = PddGoodsGetRelationResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.get.relation"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsGetRelationResponse {
    /// 外部平台商品id
    #[serde(default)]
    pub out_goods_id: String,
    /// 拼多多商品id
    #[serde(default)]
    pub pdd_goods_id: i64,
}
