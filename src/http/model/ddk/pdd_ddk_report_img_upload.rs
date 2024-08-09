//! # 多多客信息流投放备案图片上传接口
//!
//! 多多客信息流投放备案图片上传
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddDdkReportImgUploadRequest {
    /// 多多视频图片文件流
    #[serde(skip)]
    pub file: crate::http::request::FileItem,
}

impl RequestType for PddDdkReportImgUploadRequest {
    type Response = PddDdkReportImgUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.report.img.upload"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkReportImgUploadResponse {
    #[serde(default)]
    pub url: String,
}
