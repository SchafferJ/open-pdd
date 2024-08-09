//! # 判断是否对商家展示某个通知
//!
//! 判断是否对商家展示某个通知
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallNotificationTypeShowCheckRequest {
    /// 通知类型
    pub notification_type: String,
}

impl RequestType for PddMallNotificationTypeShowCheckRequest {
    type Response = PddMallNotificationTypeShowCheckResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.notification.type.show.check"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallNotificationTypeShowCheckResponse;
