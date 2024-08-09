//! 查询商品长图

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiGoodsQueryLongImagesRequest {
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
}

impl RequestType for PddAdApiGoodsQueryLongImagesRequest {
    type Response = PddAdApiGoodsQueryLongImagesResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.goods.query.long.images"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    #[serde(default, rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiGoodsQueryLongImagesResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
