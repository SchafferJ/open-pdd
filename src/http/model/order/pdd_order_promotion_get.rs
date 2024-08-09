//! # 获取订单优惠明细数据
//!
//! 查询订单优惠信息（只能获取到三个月以内的多多国际订单）。 ①.订单存在商家优惠（seller_discount）再通过此接口获取明细。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderPromotionGetRequest {
    /// 订单号列表，最多10个
    pub order_sn_list: Vec<String>,
}

impl RequestType for PddOrderPromotionGetRequest {
    type Response = PddOrderPromotionGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.promotion.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct SellerDiscountDetails {
    /// 优惠金额。单位：元
    #[serde(default)]
    pub discount: f64,
    /// 优惠描述
    #[serde(default)]
    pub discount_desc: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderPromotion {
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
    /// 商家优惠信息
    #[serde(default)]
    pub seller_discount_details: Option<Vec<SellerDiscountDetails>>,
}

#[derive(Debug, Deserialize)]
pub struct PddOrderPromotionGetResponse {
    /// 订单优惠信息列表
    #[serde(default)]
    pub order_promotion_list: Option<Vec<OrderPromotion>>,
}
