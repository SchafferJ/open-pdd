//! # 开放平台无PoiId编辑门店
//!
//! 开放平台途虎等头部商家使用无PoiId的方式编辑门店
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoStoreUpdatePostNopoiRequest {
    /// 门店营业状态
    pub business_status: i32,
    /// 一周营业时间
    pub business_week_list: Vec<i32>,
    /// 市
    pub city: String,
    /// 区
    pub district: String,
    /// 结束营业时间
    pub end_business_hour: String,
    /// 门店纬度
    pub poi_latitude: f64,
    /// 门店经度
    pub poi_longitude: f64,
    /// 省
    pub province: String,
    /// 起始营业时间
    pub start_business_hour: String,
    /// 详细地址
    pub store_address: String,
    /// 门店id
    pub store_id: i64,
    /// 门店名称
    pub store_name: String,
    /// 门店自有编号
    pub store_number: Option<String>,
    /// 门店电话
    pub store_phone: String,
    /// 门店行业（1-男女装，2-运动户外，3-服饰配件，4-厨具电器，5-汽车，8-全屋定制）
    pub trade_type: i32,
}

impl RequestType for PddMallInfoStoreUpdatePostNopoiRequest {
    type Response = PddMallInfoStoreUpdatePostNopoiResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.store.update.post.nopoi"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoStoreUpdatePostNopoiResponse;
