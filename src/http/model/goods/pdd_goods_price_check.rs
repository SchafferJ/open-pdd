//! # 商品价格核实
//!
//! 提供商品价格核实功能
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsPriceCheckRequest {
    /// 商品id，long值，大于0
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
}

impl RequestType for PddGoodsPriceCheckRequest {
    type Response = PddGoodsPriceCheckResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.price.check"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsPriceCheckResponse {
    #[serde(default)]
    pub result: i64,
}
