//! 查询商品全站推广建议出价

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrListGoodsBidSuggestRequest {
    /// 商品id列表
    #[serde(rename = "goodsIds")]
    pub goods_ids: Vec<i64>,
}

impl RequestType for PddAdApiUnitTrListGoodsBidSuggestRequest {
    type Response = PddAdApiUnitTrListGoodsBidSuggestResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.list.goods.bid.suggest"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 建议出价方式 1-目标roi 2-成交出价
    #[serde(default, rename = "defaultBidType")]
    pub default_bid_type: i32,
    /// 商品id
    #[serde(default, rename = "goodsId")]
    pub goods_id: i64,
    /// 建议成交出价，是否有值取决于出价权限，若无权限，该值为0
    #[serde(default, rename = "suggestOptimizationBid")]
    pub suggest_optimization_bid: i64,
    /// 建议目标roi，是否有值取决于出价权限，若无权限，该值为0
    #[serde(default, rename = "suggestTargetRoi")]
    pub suggest_target_roi: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrListGoodsBidSuggestResponse {
    /// 错误代码
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    /// 错误参数
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 返回结果
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    /// 是否成功
    #[serde(default)]
    pub success: bool,
}
