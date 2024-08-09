//! # 商品上架状态设置
//!
//! 修改商品上下架状态
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSaleStatusSetRequest {
    /// 拼多多商品id
    pub goods_id: i64,
    /// 上下架状态：1:上架 0:下架
    pub is_onsale: i32,
}

impl RequestType for PddGoodsSaleStatusSetRequest {
    type Response = PddGoodsSaleStatusSetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.sale.status.set"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSaleStatusSetResponse {
    /// 是否修改成功
    #[serde(default)]
    pub is_success: bool,
}
