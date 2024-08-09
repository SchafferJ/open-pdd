//! # 多多客信息流投放备案视频上传分片初始化接口
//!
//! 多多客信息流投放备案视频上传分片初始化
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddDdkReportVideoUploadPartInitRequest {
    /// 文件对应的contentType，且必须为视频类型
    pub content_type: String,
}

impl RequestType for PddDdkReportVideoUploadPartInitRequest {
    type Response = PddDdkReportVideoUploadPartInitResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.report.video.upload.part.init"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkReportVideoUploadPartInitResponse {
    #[serde(default)]
    pub upload_sign: String,
}
