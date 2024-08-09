//! # 客户与机器人的聊天记录获取接口
//!
//! 该接口用于获取客户与机器人的聊天记录。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsCsHistoryMessageGetRequest {
    /// pdd会话id
    pub session_id: String,
}

impl RequestType for PddLogisticsCsHistoryMessageGetRequest {
    type Response = PddLogisticsCsHistoryMessageGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.cs.history.message.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageInfos {
    /// 消息id
    #[serde(default)]
    pub id: i64,
    /// 消息时间戳,样式YYYY-MM-DD HH:MM:SS
    #[serde(default)]
    pub msg_ts: String,
    /// 0为用户 1为机器人
    #[serde(default)]
    pub from_type: i32,
    /// 聊天内容，如果是图片的话，则是图片的url
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsCsHistoryMessageGetResponse {
    /// 消息列表
    #[serde(default)]
    pub message_infos: Option<Vec<MessageInfos>>,
}
