//! 查询商品全站推广建议出价权限信息

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrListGoodsBidPrivilegeRequest {
    /// 商品id列表
    #[serde(rename = "goodsIds")]
    pub goods_ids: Vec<i64>,
}

impl RequestType for PddAdApiUnitTrListGoodsBidPrivilegeRequest {
    type Response = PddAdApiUnitTrListGoodsBidPrivilegeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.list.goods.bid.privilege"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 是否支持切换出价方式
    #[serde(default, rename = "canChangeBidType")]
    pub can_change_bid_type: bool,
    /// 建议出价方式 1-目标roi 2-成交出价
    #[serde(default, rename = "defaultBidType")]
    pub default_bid_type: i32,
    /// 商品id
    #[serde(default, rename = "goodsId")]
    pub goods_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrListGoodsBidPrivilegeResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
