//! # 开放平台查询门店信息
//!
//! 开放平台根据条件查询门店信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoStoreGetRequest {
    /// 市
    pub city: Option<String>,
    /// 区
    pub district: Option<String>,
    /// 分页
    pub page_number: i32,
    /// 分页大小
    pub page_size: i32,
    /// 省
    pub province: Option<String>,
    /// 门店Id
    pub store_id: Option<i64>,
    /// 门店名称
    pub store_name: Option<String>,
    /// 门店自有编号
    pub store_number: Option<String>,
}

impl RequestType for PddMallInfoStoreGetRequest {
    type Response = PddMallInfoStoreGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.store.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct MallStoreVO {
    /// 审核状态
    #[serde(default)]
    pub audit_status: i32,
    /// 市
    #[serde(default)]
    pub city: String,
    /// 区
    #[serde(default)]
    pub district: String,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 门店纬度
    #[serde(default)]
    pub poi_latitude: f64,
    /// 门店经度
    #[serde(default)]
    pub poi_longitude: f64,
    /// 省
    #[serde(default)]
    pub province: String,
    /// 门店详细地址
    #[serde(default)]
    pub store_address: String,
    /// 门店id
    #[serde(default)]
    pub store_id: i64,
    /// 门店名称
    #[serde(default)]
    pub store_name: String,
    /// 门店自有编号
    #[serde(default)]
    pub store_number: String,
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoStoreGetResponse {
    /// 返回结构体
    #[serde(default, rename = "mallStoreVOList")]
    pub mall_store_vo_list: Option<Vec<MallStoreVO>>,
}
