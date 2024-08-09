//! # 多多礼金状态更新
//!
//! 多多客授权工具商暂停或恢复多多礼金推广
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthCashgiftStatusUpdateRequest {
    /// 多多礼金ID
    pub cash_gift_id: i64,
    /// 礼金更新类型：0-停止礼金推广，1-恢复礼金推广
    pub update_type: i32,
}

impl RequestType for PddDdkOauthCashgiftStatusUpdateRequest {
    type Response = PddDdkOauthCashgiftStatusUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.cashgift.status.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthCashgiftStatusUpdateResponse {
    /// 多多礼金ID
    #[serde(default)]
    pub cash_gift_id: i64,
}
