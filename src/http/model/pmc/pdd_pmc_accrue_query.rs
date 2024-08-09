//! # 消息队列积压数量查询
//!
//! 服务商订阅消息后，查询当前积压未消费消息数量（7日内）。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPmcAccrueQueryRequest;

impl RequestType for PddPmcAccrueQueryRequest {
    type Response = PddPmcAccrueQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.pmc.accrue.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPmcAccrueQueryResponse {
    /// 消息积压数量
    #[serde(default)]
    pub number: i64,
}
