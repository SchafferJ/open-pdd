//! # 集运图片上传接口
//!
//! 集运设备供应商上传包裹图片
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddConsoImgUploadRequest {
    /// 图片文件流
    #[serde(skip)]
    pub file: crate::http::request::FileItem,
}

impl RequestType for PddConsoImgUploadRequest {
    type Response = PddConsoImgUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.conso.img.upload"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddConsoImgUploadResponse {
    #[serde(default)]
    pub url: String,
}
