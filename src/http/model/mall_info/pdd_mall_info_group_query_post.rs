//! # 查询店铺下门店组列表
//!
//! 查询店铺下门店组列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGroupQueryPostRequest {
    /// 第几页
    pub page_number: i32,
    /// 每页多少个
    pub page_size: i32,
}

impl RequestType for PddMallInfoGroupQueryPostRequest {
    type Response = PddMallInfoGroupQueryPostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.group.query.post"
    }
}

#[derive(Debug, Deserialize)]
pub struct MallStoreGroup {
    #[serde(default)]
    pub group_id: i32,
    #[serde(default)]
    pub group_name: String,
    #[serde(default)]
    pub mall_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGroupQueryPostResponse {
    #[serde(default)]
    pub mall_store_group_list: Option<Vec<MallStoreGroup>>,
    /// 总数
    #[serde(default)]
    pub total: i32,
}
