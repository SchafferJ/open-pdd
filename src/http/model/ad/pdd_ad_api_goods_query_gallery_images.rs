//! 查询商品轮播图

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiGoodsQueryGalleryImagesRequest {
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
}

impl RequestType for PddAdApiGoodsQueryGalleryImagesRequest {
    type Response = PddAdApiGoodsQueryGalleryImagesResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.goods.query.gallery.images"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 图片高
    #[serde(default, rename = "imageHeight")]
    pub image_height: i32,
    /// 图片链接
    #[serde(default, rename = "imageUrl")]
    pub image_url: String,
    /// 图片宽
    #[serde(default, rename = "imageWidth")]
    pub image_width: i32,
    /// 标识轮播图排序，最小的为主轮播图
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiGoodsQueryGalleryImagesResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
