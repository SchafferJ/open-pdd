//! # 查询店铺身份
//!
//! 确认店铺的身份
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddFdsRoleGetRequest;

impl RequestType for PddFdsRoleGetRequest {
    type Response = PddFdsRoleGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.fds.role.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddFdsRoleGetResponse {
    /// 店铺身份 0：未知身份 2：厂家
    #[serde(default)]
    pub mall_role: i32,
}
