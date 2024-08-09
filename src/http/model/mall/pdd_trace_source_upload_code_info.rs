//! # 溯源服务商上传溯源码信息
//!
//! 溯源服务商上传溯源码信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct SerialNum {
    /// 溯源码（处理后）
    pub encoded_serial_num: String,
    /// 溯源码
    pub serial_num: String,
}

#[derive(Debug, Serialize)]
pub struct PddTraceSourceUploadCodeInfoRequest {
    /// 溯源码列表
    pub serial_num_list: Vec<SerialNum>,
}

impl RequestType for PddTraceSourceUploadCodeInfoRequest {
    type Response = PddTraceSourceUploadCodeInfoResponse;

    fn get_type(&self) -> &'static str {
        "pdd.trace.source.upload.code.info"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTraceSourceUploadCodeInfoResponse;
