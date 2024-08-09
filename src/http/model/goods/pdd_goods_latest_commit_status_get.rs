//! # 批量goodsId查询最新的审核状态
//!
//! 批量goodsId查询最新的审核状态
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsLatestCommitStatusGetRequest {
    /// 商品id(不超过100个)
    pub goods_id_list: Vec<i64>,
}

impl RequestType for PddGoodsLatestCommitStatusGetRequest {
    type Response = PddGoodsLatestCommitStatusGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.latest.commit.status.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 草稿状态
    #[serde(default)]
    pub check_status: i32,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 驳回原因
    #[serde(default)]
    pub reject_comment: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsLatestCommitStatusGetResponse {
    /// list
    #[serde(default)]
    pub list: Option<Vec<List>>,
}
