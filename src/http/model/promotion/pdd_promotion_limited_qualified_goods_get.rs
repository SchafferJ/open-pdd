//! # 限时限量购可选商品查询接口
//!
//! 查询可参与限时限量购活动的商品
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionLimitedQualifiedGoodsGetRequest {
    /// 商品id列表
    pub goods_id_list: Option<Vec<i64>>,
    /// TRUE-仅查询可选商品（满足活动资格商品）数据；FALSE-查询不可选商品数据
    pub is_valid: bool,
    /// 页码
    pub page_no: Option<i32>,
    /// 每页查询数
    pub page_size: Option<i32>,
}

impl RequestType for PddPromotionLimitedQualifiedGoodsGetRequest {
    type Response = PddPromotionLimitedQualifiedGoodsGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.limited.qualified.goods.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionLimitedQualifiedGoodsGetResponse {
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
}
