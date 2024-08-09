//! # 图片上传到图片空间
//!
//! 图片上传到图片空间
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddGoodsFilespaceImageUploadRequest {
    /// 图片文件流
    #[serde(skip)]
    pub file: crate::http::request::FileItem,
}

impl RequestType for PddGoodsFilespaceImageUploadRequest {
    type Response = PddGoodsFilespaceImageUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.filespace.image.upload"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsFilespaceImageUploadResponse {
    /// 文件id
    #[serde(default)]
    pub file_id: i64,
    /// url
    #[serde(default)]
    pub url: String,
}
