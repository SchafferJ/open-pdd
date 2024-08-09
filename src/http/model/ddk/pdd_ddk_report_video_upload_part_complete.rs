//! # 多多客信息流投放备案视频上传分片完成接口
//!
//! 多多客信息流投放备案视频上传分片完成
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddDdkReportVideoUploadPartCompleteRequest {
    /// 标记本次大文件上传的id（init阶段的返回值）
    pub upload_sign: String,
}

impl RequestType for PddDdkReportVideoUploadPartCompleteRequest {
    type Response = PddDdkReportVideoUploadPartCompleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.report.video.upload.part.complete"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkReportVideoUploadPartCompleteResponse {
    /// 创建的视频资源对应的vid
    #[serde(default)]
    pub url: String,
}
