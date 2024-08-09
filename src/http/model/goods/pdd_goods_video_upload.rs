//! # 商品视频上传接口
//!
//! 商品视频上传接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddGoodsVideoUploadRequest {
    /// 视频文件,为文件流
    #[serde(skip)]
    pub file: crate::http::request::FileItem,
}

impl RequestType for PddGoodsVideoUploadRequest {
    type Response = PddGoodsVideoUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.video.upload"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsVideoUploadResponse {
    /// 文件id
    #[serde(default)]
    pub id: i64,
    /// 文件url
    #[serde(default)]
    pub url: String,
    /// 审核状态
    #[serde(default)]
    pub status: i32,
}
