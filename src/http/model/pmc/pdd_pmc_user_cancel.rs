//! # 取消用户的消息服务
//!
//! 取消用户的消息服务
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPmcUserCancelRequest {
    /// 用户唯一id
    pub owner_id: String,
}

impl RequestType for PddPmcUserCancelRequest {
    type Response = PddPmcUserCancelResponse;

    fn get_type(&self) -> &'static str {
        "pdd.pmc.user.cancel"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPmcUserCancelResponse {
    /// 是否成功
    #[serde(default)]
    pub is_success: bool,
}
