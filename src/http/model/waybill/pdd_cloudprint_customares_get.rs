//! # 获取商家的自定义区模板信息
//!
//! 供isv使用，获取商家的自定义区的模板信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddCloudprintCustomaresGetRequest {
    /// 即pdd.cloudprint.stdtemplates.get接口返回的standard_template_id
    pub template_id: i32,
}

impl RequestType for PddCloudprintCustomaresGetRequest {
    type Response = PddCloudprintCustomaresGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloudprint.customares.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Key{}

#[derive(Debug, Deserialize)]
pub struct Datas {
    /// custom_area_id
    #[serde(default)]
    pub custom_area_id: i64,
    /// custom_area_name
    #[serde(default)]
    pub custom_area_name: String,
    /// custom_area_url
    #[serde(default)]
    pub custom_area_url: String,
    /// keys
    #[serde(default)]
    pub keys: Vec<Key>,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// datas
    #[serde(default)]
    pub datas: Option<Vec<Datas>>,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudprintCustomaresGetResponse {
    /// result
    #[serde(default)]
    pub result: Option<Result>,
}
