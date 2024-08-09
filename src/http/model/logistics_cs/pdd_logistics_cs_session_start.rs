//! # 物流客服系统创建同步会话状态接口
//!
//! 该接口用于物流客服系统创建以及同步会话状态。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsCsSessionStartRequest {
    /// pdd会话id
    pub session_id: String,
    /// 物流公司会话id
    pub wp_session_id: String,
    /// 样式YYYY-MM-DD HH:MM:SS
    pub action_time: String,
    /// 可选值：1：已分配 2：排队中 3：分配异常
    pub biz_type: i32,
    /// 客服id，biz_type为1时必填
    pub dealer_id: Option<String>,
    /// 队列id，biz_type为1时必填
    pub queue_id: Option<String>,
    /// 网点名，biz_type为1时必填
    pub queue_name: Option<String>,
    /// 排队位置，biz_type为2时必填
    pub queue_index: Option<i32>,
    /// 分配遇到的异常，示例：33222，biz_type为3时不为空
    pub exception_code: Option<i32>,
    /// 物流客服系统遇到的异常，biz_type为3时不为空
    pub exception_msg: Option<String>,
    /// 接待的网点地址，biz_type为1时必填，示例: ”河南省”
    pub queue_address: Option<String>,
}

impl RequestType for PddLogisticsCsSessionStartRequest {
    type Response = PddLogisticsCsSessionStartResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.cs.session.start"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsCsSessionStartResponse {
    /// 是否成功
    #[serde(default)]
    pub is_success: bool,
}
