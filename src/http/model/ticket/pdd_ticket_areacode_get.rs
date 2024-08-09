//! # 旅游门票区域编码查询
//!
//! 供应商获取pdd的区域编码
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddTicketAreacodeGetRequest;

impl RequestType for PddTicketAreacodeGetRequest {
    type Response = PddTicketAreacodeGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.areacode.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Area {
    #[serde(default)]
    pub code: i32,
    #[serde(default)]
    pub level: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub parent_code: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddTicketAreacodeGetResponse {
    #[serde(default)]
    pub area_list: Option<Vec<Area>>,
}
