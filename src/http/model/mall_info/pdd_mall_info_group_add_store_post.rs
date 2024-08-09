//! # 门店组添加门店
//!
//! 门店组添加门店
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGroupAddStorePostRequest {
    /// 店铺ID
    pub group_id: i64,
    /// 门店ID列表
    pub store_id_list: Vec<i64>,
}

impl RequestType for PddMallInfoGroupAddStorePostRequest {
    type Response = PddMallInfoGroupAddStorePostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.group.add.store.post"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGroupAddStorePostResponse {
    #[serde(default)]
    pub is_success: bool,
}
