//! # 删除草稿接口
//!
//! 删除草稿接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDeleteDraftCommitRequest {
    /// 草稿id
    pub goods_commit_id: i64,
    /// 商品id
    pub goods_id: i64,
}

impl RequestType for PddDeleteDraftCommitRequest {
    type Response = PddDeleteDraftCommitResponse;

    fn get_type(&self) -> &'static str {
        "pdd.delete.draft.commit"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDeleteDraftCommitResponse;
