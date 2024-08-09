//! # 批量绑定推广位的媒体id
//!
//! 批量对pid与媒体id进行绑定
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthPidMediaidBindRequest {
    /// 媒体id
    pub media_id: i64,
    /// 推广位列表，例如：["60005_612"]，最多支持同时传入1000个
    pub pid_list: Vec<String>,
}

impl RequestType for PddDdkOauthPidMediaidBindRequest {
    type Response = PddDdkOauthPidMediaidBindResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.pid.mediaid.bind"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 绑定结果信息提示
    #[serde(default)]
    pub msg: String,
    /// 绑定结果
    #[serde(default)]
    pub result: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthPidMediaidBindResponse {
    /// 绑定结果
    #[serde(default)]
    pub result: Option<Result>,
}
