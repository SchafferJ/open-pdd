//! # 多多客信息流投放备案视频上传接口
//!
//! 多多客信息流投放备案视频上传,上传视频大小有限制,单个文件超过20M需要走分片上传
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddDdkReportVideoUploadRequest {
    /// 多多客信息流投放备案视频文件流
    #[serde(skip)]
    pub file: crate::http::request::FileItem,
}

impl RequestType for PddDdkReportVideoUploadRequest {
    type Response = PddDdkReportVideoUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.report.video.upload"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkReportVideoUploadResponse {
    /// 创建的视频资源对应的vid
    #[serde(default)]
    pub url: String,
}
