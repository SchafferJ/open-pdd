//! # 修改商品sku价格
//!
//! 修改商品sku价格
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SkuPrice {
    /// 拼团购买价格（单位分）
    pub group_price: Option<i64>,
    /// sku上架状态，0-已下架，1-上架中
    pub is_onsale: Option<i32>,
    /// 单独购买价格（单位分）
    pub single_price: Option<i64>,
    /// sku标识
    pub sku_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsSkuPriceUpdateRequest {
    /// 商品id
    pub goods_id: i64,
    /// 是否获取商品发布警告信息，默认为忽略
    pub ignore_edit_warn: Option<bool>,
    /// 参考价 （单位分）
    pub market_price: Option<i64>,
    /// 参考价 （单位元）
    pub market_price_in_yuan: Option<String>,
    /// 待修改的sku价格
    pub sku_price_list: Vec<SkuPrice>,
    /// 提交后上架状态，0:上架,1:保持原样
    pub sync_goods_operate: Option<i32>,
    /// 满2件折扣，可选范围0-100, 0表示取消，95表示95折，设置需先查询规则接口获取实际可填范围
    pub two_pieces_discount: Option<i32>,
}

impl RequestType for PddGoodsSkuPriceUpdateRequest {
    type Response = PddGoodsSkuPriceUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.sku.price.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSkuPriceUpdateResponse {
    #[serde(default)]
    pub goods_commit_id: i64,
    /// 是否成功
    #[serde(default)]
    pub is_success: bool,
}
