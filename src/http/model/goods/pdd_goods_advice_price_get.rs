//! # 商品建议价格获取接口
//!
//! 商家可通过此接口查询店铺内所有上架商品的建议价格
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 页码，默认1
    pub page: Option<i32>,
    /// 每页数量，默认100，最大100
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsAdvicePriceGetRequest {
    /// 获取商品建议价格请求参数
    pub request: Request,
}

impl RequestType for PddGoodsAdvicePriceGetRequest {
    type Response = PddGoodsAdvicePriceGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.advice.price.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct AdvicePrice {
    /// sku建议价，单位为分
    #[serde(default)]
    pub advice_price: i64,
    /// 商品ID
    #[serde(default)]
    pub goods_id: i64,
    /// 商品标题
    #[serde(default)]
    pub goods_name: String,
    /// sku拼单价，单位为分
    #[serde(default)]
    pub group_price: i64,
    /// skuID
    #[serde(default)]
    pub sku_id: i64,
    /// sku名
    #[serde(default)]
    pub sku_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsAdvicePriceGetResponse {
    /// 商品建议价列表
    #[serde(default)]
    pub advice_price_list: Option<Vec<AdvicePrice>>,
    /// 商品建议价数据总数
    #[serde(default)]
    pub total: i32,
}
