//! # ISV物流轨迹推送消息订阅接口
//!
//! 商家在ISV发货成功之后，ISV通过调用订阅接口订阅轨迹推送消息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsIsvTraceNotifySubRequest {
    /// 快递公司编码
    pub ship_code: String,
    /// 收件人手机
    pub tel: String,
    /// 快递单号
    pub track_no: String,
}

impl RequestType for PddLogisticsIsvTraceNotifySubRequest {
    type Response = PddLogisticsIsvTraceNotifySubResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.isv.trace.notify.sub"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsIsvTraceNotifySubResponse {
    /// 是否成功，false-失败，true-成功
    #[serde(default)]
    pub is_success: bool,
}
