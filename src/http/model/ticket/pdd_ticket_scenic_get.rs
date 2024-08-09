//! # 旅游门票拼多多景区编码查询
//!
//! isv查询pdd景区
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddTicketScenicGetRequest {
    /// 城市编码
    pub city_code: Option<i64>,
    /// 定位类型 1.百度 2.google
    pub location_type: i32,
    /// 拼多多景区 ID
    pub scenic_id: Option<i64>,
    /// 景区简称（至少两个字）
    pub scenic_name: String,
}

impl RequestType for PddTicketScenicGetRequest {
    type Response = PddTicketScenicGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.scenic.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Area {
    /// 纬度
    #[serde(default)]
    pub latitude: f64,
    /// 定位类型 1.百度 2.google
    #[serde(default)]
    pub location_type: i32,
    /// 经度
    #[serde(default)]
    pub longitude: f64,
    /// 拼多多景区编码
    #[serde(default)]
    pub scenic_id: i64,
    /// 拼多多景区名称
    #[serde(default)]
    pub scenic_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddTicketScenicGetResponse {
    #[serde(default)]
    pub area_list: Option<Vec<Area>>,
}
