//! 查询可用商品列表（分页）

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiGoodsQueryPageRequest {
    /// 商品名称
    #[serde(rename = "goodsName")]
    pub goods_name: Option<String>,
    /// 分页查询，查询第几页
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    /// 分页查询，每页的大小
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// 计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
}

impl RequestType for PddAdApiGoodsQueryPageRequest {
    type Response = PddAdApiGoodsQueryPageResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.goods.query.page"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct ResultResponse {
    /// 商品类目Id
    #[serde(default, rename = "catId")]
    pub cat_id: i64,
    /// 商品类目名称
    #[serde(default, rename = "catName")]
    pub cat_name: String,
    /// 商品Id
    #[serde(default, rename = "goodsId")]
    pub goods_id: i64,
    /// 商品名称
    #[serde(default, rename = "goodsName")]
    pub goods_name: String,
    /// 商品最小团购价
    #[serde(default, rename = "minGroupPrice")]
    pub min_group_price: i64,
    /// 商品库存
    #[serde(default)]
    pub quantity: i64,
    /// 商品销量
    #[serde(default, rename = "soldQuantity")]
    pub sold_quantity: i64,
    /// 商品图片链接
    #[serde(default, rename = "thumbUrl")]
    pub thumb_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    #[serde(default)]
    pub result: Option<Vec<ResultResponse>>,
    #[serde(default)]
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiGoodsQueryPageResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
