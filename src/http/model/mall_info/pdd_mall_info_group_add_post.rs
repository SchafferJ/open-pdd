//! # 新增门店组
//!
//! 新增门店组
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGroupAddPostRequest {
    /// 门店组名称
    pub group_name: String,
}

impl RequestType for PddMallInfoGroupAddPostRequest {
    type Response = PddMallInfoGroupAddPostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.group.add.post"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGroupAddPostResponse {
    /// 门店组id
    #[serde(default)]
    pub group_id: i32,
}
