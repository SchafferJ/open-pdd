//! # 创建店铺首页优惠券批次接口
//!
//! 创建店铺首页优惠券批次接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionHomeCouponCreateRequest {
    /// 描述
    pub batch_desc: String,
    /// 开始时间，指到格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)的总毫秒数
    pub batch_start_time: i64,
    /// 结束时间，指到格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)的总毫秒数
    pub batch_end_time: i64,
    /// 优惠金额	单位: 分
    pub discount: i64,
    /// 使用优惠的订单最小金额	单位: 分
    pub min_order_amount: i64,
    /// 可领取数量
    pub init_quantity: i64,
    /// 每个用户限领张数
    pub user_limit: i64,
}

impl RequestType for PddPromotionHomeCouponCreateRequest {
    type Response = PddPromotionHomeCouponCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.home.coupon.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionHomeCouponCreateResponse {
    /// 批次id
    #[serde(default)]
    pub batch_id: i64,
}
