//! # 删除店铺门店
//!
//! 删除店铺门店
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoStoreDeletePostRequest {
    /// 门店ID列表
    pub store_id_list: Vec<i64>,
}

impl RequestType for PddMallInfoStoreDeletePostRequest {
    type Response = PddMallInfoStoreDeletePostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.store.delete.post"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoStoreDeletePostResponse {
    #[serde(default)]
    pub is_success: bool,
}
