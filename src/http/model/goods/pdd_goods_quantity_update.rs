//! # 商品库存更新接口
//!
//! 修改商品sku库存，在资源位上的商品不能减少库存
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsQuantityUpdateRequest {
    /// 商品id
    pub goods_id: i64,
    /// 库存修改值。当全量更新库存时，quantity必须为大于等于0的正整数；当增量更新库存时，quantity为整数，可小于等于0。若增量更新时传入的库存为负数，则负数与实际库存之和不能小于0。比如当前实际库存为1，传入增量更新quantity=-1，库存改为0
    pub quantity: i64,
    /// sku_id和outer_id必填一个，优先使用sku_id
    pub sku_id: Option<i64>,
    /// sku商家编码，如果sku_id未填，则使用outer_id
    pub outer_id: Option<String>,
    /// 库存更新方式，可选。1为全量更新，2为增量更新。如果不填，默认为全量更新
    pub update_type: Option<i32>,
}

impl RequestType for PddGoodsQuantityUpdateRequest {
    type Response = PddGoodsQuantityUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.quantity.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsQuantityUpdateResponse {
    /// true
    #[serde(default)]
    pub is_success: bool,
}
