//! # 查询店铺是否签署多多进宝协议接口
//!
//! 查询店铺是否签署多多进宝协议接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallCpsProtocolStatusQueryRequest;

impl RequestType for PddMallCpsProtocolStatusQueryRequest {
    type Response = PddMallCpsProtocolStatusQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.cps.protocol.status.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallCpsProtocolStatusQueryResponse {
    /// 是否签署协议
    #[serde(default)]
    pub status: bool,
}
