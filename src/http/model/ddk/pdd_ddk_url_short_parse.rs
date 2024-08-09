//! # 多多进宝推广短链解析
//!
//! 多多进宝推广短链解析
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkUrlShortParseRequest {
    /// 需要解析出长链的多多进宝短连接，仅支持短链接（即为pdd.ddk.goods.promotion.url.generate接口生成的短链）
    pub original_url: String,
}

impl RequestType for PddDdkUrlShortParseRequest {
    type Response = PddDdkUrlShortParseResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.url.short.parse"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkUrlShortParseResponse;
