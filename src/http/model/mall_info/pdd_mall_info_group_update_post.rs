//! # 编辑门店组
//!
//! 编辑门店组
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGroupUpdatePostRequest {
    /// 门店组id
    pub group_id: i32,
    /// 门店组名称
    pub group_name: String,
}

impl RequestType for PddMallInfoGroupUpdatePostRequest {
    type Response = PddMallInfoGroupUpdatePostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.group.update.post"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGroupUpdatePostResponse {
    #[serde(default)]
    pub result: bool,
}
