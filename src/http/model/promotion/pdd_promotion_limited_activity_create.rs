//! # 限时限量购活动创建接口
//!
//! 创建限时限量购活动（包括限时折扣和限量折扣）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SkuPrice {
    /// sku活动价格
    pub activity_price: i64,
    /// skuid
    pub sku_id: i64,
}

#[derive(Debug, Serialize)]
pub struct Request {
    /// 活动名称
    pub activity_name: String,
    /// 活动类型
    pub activity_type: i32,
    /// 折扣比例，实际折扣为：discount/1000。例：300，表示3折
    pub discount: Option<i64>,
    /// 限时折扣必填。结束时间（单位：s）
    pub end_time: Option<i64>,
    /// 商品id
    pub goods_id: i64,
    /// 活动库存的数量（限量活动时，必填）
    pub quantity: Option<i64>,
    /// 参与活动的sku数据列表
    pub sku_price_list: Vec<SkuPrice>,
    /// 限时折扣必填。开始时间（单位：s）
    pub start_time: Option<i64>,
    /// 用户限购数量(0:不限购)
    pub user_activity_limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddPromotionLimitedActivityCreateRequest {
    /// 创建请求
    pub request: Vec<Request>,
}

impl RequestType for PddPromotionLimitedActivityCreateRequest {
    type Response = PddPromotionLimitedActivityCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.limited.activity.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionLimitedActivityCreateResponse {
    /// 创建失败时的失败原因
    #[serde(default)]
    pub fail_reason: String,
    /// 商品id
    #[serde(default)]
    pub goods_id: String,
    /// 商品是否创建成功。0代表成功，非0代表失败
    #[serde(default)]
    pub status: i32,
}
