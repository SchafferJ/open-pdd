//! # 商品图片上传接口
//!
//! 商品图片上传
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddGoodsImgUploadRequest {
    /// 商品图片文件流
    #[serde(skip)]
    pub file: crate::http::request::FileItem,
}

impl RequestType for PddGoodsImgUploadRequest {
    type Response = PddGoodsImgUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.img.upload"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::FILE
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsImgUploadResponse {
    #[serde(default)]
    pub url: String,
}
