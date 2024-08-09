//! # 轨迹订阅接口
//!
//! 拼多多向物流公司订阅指定运单号的物流轨迹详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddLogisticsCoTrackSubRequest {
    /// 快递公司伙伴ID
    pub ship_id: Option<String>,
    /// 消息体
    pub data: Option<String>,
}

impl RequestType for PddLogisticsCoTrackSubRequest {
    type Response = PddLogisticsCoTrackSubResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.co.track.sub"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsCoTrackSubResponse;
