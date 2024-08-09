//! # 多多客信息流投放备案视频上传分片上传接口
//!
//! 多多客信息流投放备案视频上传分片上传上传接口，每个分片建议不超过20M
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddDdkReportVideoUploadPartRequest {
    /// 当前分片的文件流
    #[serde(skip)]
    pub part_file: crate::http::request::FileItem,
    /// 当前分片编号名，从1开始
    pub part_num: String,
    /// 标记本次大文件上传的id（init阶段的返回值）
    pub upload_sign: String,
}

impl RequestType for PddDdkReportVideoUploadPartRequest {
    type Response = PddDdkReportVideoUploadPartResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.report.video.upload.part"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkReportVideoUploadPartResponse {
    /// 表示本次成功上传的part number
    #[serde(default)]
    pub uploaded_part_num: i32,
}
