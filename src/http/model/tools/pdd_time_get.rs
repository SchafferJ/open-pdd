//! # 获取拼多多系统时间
//!
//! 获取拼多多系统时间
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddTimeGetRequest;

impl RequestType for PddTimeGetRequest {
    type Response = PddTimeGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.time.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTimeGetResponse {
    /// 拼多多系统时间
    #[serde(default)]
    pub time: String,
}
