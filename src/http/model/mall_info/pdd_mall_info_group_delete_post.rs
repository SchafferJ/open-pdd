//! # 删除门店组
//!
//! 删除门店组
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGroupDeletePostRequest {
    /// 门店组id列表
    pub group_id_list: Vec<i32>,
}

impl RequestType for PddMallInfoGroupDeletePostRequest {
    type Response = PddMallInfoGroupDeletePostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.group.delete.post"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGroupDeletePostResponse {
    /// 操作是否成功
    #[serde(default)]
    pub result: bool,
}
