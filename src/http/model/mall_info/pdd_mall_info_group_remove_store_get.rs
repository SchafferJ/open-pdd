//! # 门店组删除门店
//!
//! 门店组删除门店
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGroupRemoveStoreGetRequest {
    /// 店铺ID
    pub group_id: i64,
    /// 门店ID列表
    pub store_id_list: Vec<i64>,
}

impl RequestType for PddMallInfoGroupRemoveStoreGetRequest {
    type Response = PddMallInfoGroupRemoveStoreGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.group.remove.store.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGroupRemoveStoreGetResponse {
    #[serde(default)]
    pub is_success: bool,
}
