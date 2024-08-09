//! # 创建店铺门店
//!
//! 创建店铺门店
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoStoreCreatePostRequest {
    /// 门店营业状态
    pub business_status: i32,
    /// 营业天数
    pub business_week_list: Vec<i32>,
    /// 省市区三级地址-市名
    pub city: String,
    /// 省市区三级地址-区名
    pub district: String,
    /// 结束营业时间段
    pub end_business_hour: String,
    /// 腾讯地图POI信息ID
    pub poi_id: String,
    /// 门店纬度
    pub poi_latitude: f64,
    /// 门店经度
    pub poi_longitude: f64,
    /// 省市区三级地址-省名
    pub province: String,
    /// 起始营业时间段
    pub start_business_hour: String,
    /// 门店名称
    pub store_name: String,
    /// 门店自有编号
    pub store_number: Option<String>,
    /// 门店电话
    pub store_phone: String,
    /// 门店行业（1-男女装，2-运动户外，3-服饰配件，4-厨具电器，5-汽车，8-全屋定制）
    pub trade_type: i32,
}

impl RequestType for PddMallInfoStoreCreatePostRequest {
    type Response = PddMallInfoStoreCreatePostResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.store.create.post"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoStoreCreatePostResponse {
    #[serde(default)]
    pub is_success: bool,
}
