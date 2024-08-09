//! # 根据防伪码ID获取溯源商品信息
//!
//! 根据溯源码ID获取溯源商品信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct Params {
    /// 接口调用账号（由平台分配）
    pub userid: String,
    /// 请求时间戳，10分钟有效，格式：yyyy-MM-dd HH:mm:ss
    pub timestamp: String,
    /// 签名
    pub sign: String,
    /// 防伪溯源码ID
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct PddTraceSourceQueryGoodsInfoRequest {
    /// 请求方法
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// 请求参数
    pub params: Params,
}

impl RequestType for PddTraceSourceQueryGoodsInfoRequest {
    type Response = PddTraceSourceQueryGoodsInfoResponse;

    fn get_type(&self) -> &'static str {
        "pdd.trace.source.query.goods.info"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTraceSourceQueryGoodsInfoResponse;
