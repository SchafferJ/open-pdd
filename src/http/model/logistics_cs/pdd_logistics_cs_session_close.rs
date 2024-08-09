//! # 客服关闭会话接口
//!
//! 该接口用于客服关闭会话
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsCsSessionCloseRequest {
    /// pdd会话id
    pub session_id: String,
    /// 物流公司会话id
    pub wp_session_id: String,
    /// 样式YYYY-MM-DD HH:MM:SS
    pub action_time: String,
}

impl RequestType for PddLogisticsCsSessionCloseRequest {
    type Response = PddLogisticsCsSessionCloseResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.cs.session.close"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsCsSessionCloseResponse {
    /// 是否成功
    #[serde(default)]
    pub is_success: bool,
}
