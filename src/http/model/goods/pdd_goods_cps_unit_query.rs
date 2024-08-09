//! # 查询商品推广API
//!
//! 查询商品推广API
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCpsUnitQueryRequest {
    /// 商品id
    pub goods_id: i64,
}

impl RequestType for PddGoodsCpsUnitQueryRequest {
    type Response = PddGoodsCpsUnitQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cps.unit.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCpsUnitQueryResponse {
    /// coupon_end_time
    #[serde(default)]
    pub coupon_end_time: String,
    /// coupon_id
    #[serde(default)]
    pub coupon_id: i64,
    /// coupon_start_time
    #[serde(default)]
    pub coupon_start_time: String,
    /// discount
    #[serde(default)]
    pub discount: i32,
    /// init_quantity
    #[serde(default)]
    pub init_quantity: i64,
    /// remain_quantity
    #[serde(default)]
    pub remain_quantity: i64,
}
