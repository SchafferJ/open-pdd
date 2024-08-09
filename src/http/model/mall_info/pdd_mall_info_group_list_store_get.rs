//! # 查询门店组下门店
//!
//! 查询门店组下门店
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGroupListStoreGetRequest {
    /// 门店组ID
    pub group_id: i64,
    /// 分页页码
    pub page_number: i32,
    /// 分页大小
    pub page_size: i32,
}

impl RequestType for PddMallInfoGroupListStoreGetRequest {
    type Response = PddMallInfoGroupListStoreGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.group.list.store.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct MallStoreFinalVo {
    /// 门店经营状态
    #[serde(default, rename = "businessStatus")]
    pub business_status: i32,
    /// 省市区三级地址-市
    #[serde(default)]
    pub city: String,
    /// 省市区三级地址-区
    #[serde(default)]
    pub district: String,
    #[serde(default)]
    pub exists: bool,
    /// 省市区三级地址-省
    #[serde(default)]
    pub province: String,
    /// 门店地址
    #[serde(default, rename = "storeAddress")]
    pub store_address: String,
    /// 门店ID
    #[serde(default, rename = "storeId")]
    pub store_id: i64,
    /// 店铺名称
    #[serde(default, rename = "storeName")]
    pub store_name: String,
    /// 门店自有编号
    #[serde(default, rename = "storeNumber")]
    pub store_number: String,
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGroupListStoreGetResponse {
    /// 门店列表
    #[serde(default)]
    pub mall_store_final_vo_list: Option<Vec<MallStoreFinalVo>>,
    /// 总数
    #[serde(default)]
    pub total: i32,
}
