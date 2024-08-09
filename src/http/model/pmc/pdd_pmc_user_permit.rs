//! # 为已授权的用户开通消息服务
//!
//! 为已授权的用户开通消息服务
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPmcUserPermitRequest {
    /// 消息主题列表，用半角逗号分隔。当用户订阅的topic是应用订阅的子集时才需要设置，不设置表示继承应用所订阅的所有topic，一般情况建议不要设置。
    pub topics: Option<String>,
}

impl RequestType for PddPmcUserPermitRequest {
    type Response = PddPmcUserPermitResponse;

    fn get_type(&self) -> &'static str {
        "pdd.pmc.user.permit"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPmcUserPermitResponse {
    /// 是否成功
    #[serde(default)]
    pub is_success: bool,
}
