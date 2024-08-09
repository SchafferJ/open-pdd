//! # 删除商品接口
//!
//! 删除商品接口,只能删除下架的商品
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDeleteGoodsCommitRequest {
    /// 商品id 列表(List<Long>) json string，例：[1,2]，一次操作数量请小于50
    pub goods_ids: Vec<i64>,
}

impl RequestType for PddDeleteGoodsCommitRequest {
    type Response = PddDeleteGoodsCommitResponse;

    fn get_type(&self) -> &'static str {
        "pdd.delete.goods.commit"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDeleteGoodsCommitResponse;
