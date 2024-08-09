//! # 草稿状态查询接口
//!
//! 查询店铺的商品草稿列表状态
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCommitStatusGetRequest {
    /// goods_commit_id列表
    pub goods_commit_id_list: Vec<i64>,
}

impl RequestType for PddGoodsCommitStatusGetRequest {
    type Response = PddGoodsCommitStatusGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.commit.status.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 状态
    #[serde(default)]
    pub check_status: i32,
    /// 草稿id
    #[serde(default)]
    pub goods_commit_id: i64,
    /// 驳回原因
    #[serde(default)]
    pub reject_comment: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCommitStatusGetResponse {
    /// list
    #[serde(default)]
    pub list: Option<Vec<List>>,
}
