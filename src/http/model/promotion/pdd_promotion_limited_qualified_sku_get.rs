//! # 限时限量购可选sku查询接口
//!
//! 查询可参加限时限量购活动的sku列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionLimitedQualifiedSkuGetRequest {
    /// 商品id
    pub goods_id: i64,
}

impl RequestType for PddPromotionLimitedQualifiedSkuGetRequest {
    type Response = PddPromotionLimitedQualifiedSkuGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.limited.qualified.sku.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionLimitedQualifiedSkuGetResponse {
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 不可选原因
    #[serde(default)]
    pub invalid_reason: String,
    /// skuid
    #[serde(default)]
    pub sku_id: i64,
    /// sku是否可以设置营销工具活动价。0-sku有效，1-sku需上架，2-sku库存需≥3，3-sku参考价不符合规则
    #[serde(default)]
    pub valid_status: i32,
}
