//! # 服务时间查询接口
//!
//! 服务时间查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct Request {
    /// 省名称
    #[serde(rename = "provName")]
    pub prov_name: Option<String>,
    /// 市名称
    #[serde(rename = "cityName")]
    pub city_name: Option<String>,
    /// 区名称
    #[serde(rename = "districtName")]
    pub district_name: Option<String>,
    /// 街道名称
    #[serde(rename = "streetName")]
    pub street_name: Option<String>,
    /// 寄件类型
    #[serde(rename = "postType")]
    pub post_type: Option<String>,
    /// 收件省名称
    #[serde(rename = "receiveProvName")]
    pub receive_prov_name: Option<String>,
    /// 收件市名称
    #[serde(rename = "receiveCityName")]
    pub receive_city_name: Option<String>,
    /// 收件区名称
    #[serde(rename = "receiveDistrictName")]
    pub receive_district_name: Option<String>,
    /// 收件街道名称
    #[serde(rename = "receiveStreetName")]
    pub receive_street_name: Option<String>,
    /// 收件详细地址
    #[serde(rename = "receiveAddrDetail")]
    pub receive_addr_detail: Option<String>,
    /// 扩展信息； options如果不存在，说明不需要对发货地收货地校验
    pub attributes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PddServiceMailServiceTimeQueryRequest {
    /// 请求参数
    pub request: Option<Request>,
}

impl RequestType for PddServiceMailServiceTimeQueryRequest {
    type Response = PddServiceMailServiceTimeQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.service.mail.service.time.query"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct ServiceTimes {
    /// 开始服务时间
    #[serde(default, rename = "startTime")]
    pub start_time: String,
    /// 结束服务时间
    #[serde(default, rename = "endTime")]
    pub end_time: String,
    /// 服务日期
    #[serde(default, rename = "serviceDate")]
    pub service_date: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceOptions {
    #[serde(default, rename = "canSend")]
    pub can_send: bool,
    #[serde(default, rename = "canReceivce")]
    pub can_receivce: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddServiceMailServiceTimeQueryResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default)]
    pub flag: bool,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default, rename = "serviceTimes")]
    pub service_times: Option<Vec<ServiceTimes>>,
    #[serde(default, rename = "serviceOptions")]
    pub service_options: Option<ServiceOptions>,
}
