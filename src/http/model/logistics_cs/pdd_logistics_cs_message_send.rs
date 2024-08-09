//! # 客服给发消息接口
//!
//! 该接口用于客服给客户发消息，发消息的前提是有客服分配。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsCsMessageSendRequest {
    /// pdd会话id
    pub session_id: String,
    /// 物流公司会话id
    pub wp_session_id: String,
    /// 样式YYYY-MM-DD HH:MM:SS
    pub action_time: String,
    /// 0：文本1：图片
    pub message_type: i32,
    /// message_type为0时不为空
    pub text: Option<String>,
    /// message_type为1时不为空
    pub attach: Option<String>,
    /// message_type为1时不为空
    pub preview: Option<String>,
}

impl RequestType for PddLogisticsCsMessageSendRequest {
    type Response = PddLogisticsCsMessageSendResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.cs.message.send"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsCsMessageSendResponse {
    /// 是否成功
    #[serde(default)]
    pub is_success: bool,
}
