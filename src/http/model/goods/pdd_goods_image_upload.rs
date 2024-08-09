//! # 商品图片上传接口
//!
//! 商品图片上传服务（参数拼接成json串）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsImageUploadRequest {
    /// 支持格式有：jpg/jpeg、png等图片格式，注意入参图片必须转码为base64编码
    pub image: String,
}

impl RequestType for PddGoodsImageUploadRequest {
    type Response = PddGoodsImageUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.image.upload"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsImageUploadResponse {
    /// 返回图片url
    #[serde(default)]
    pub image_url: String,
}
