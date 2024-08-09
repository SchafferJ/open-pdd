//! # 创建无门槛商品劵批次接口
//!
//! 创建无门槛商品劵批次接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionGoodsCouponCreateRequest {
    /// 描述
    pub batch_desc: String,
    /// 开始时间，指到格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)的总毫秒数
    pub batch_start_time: i64,
    /// 结束时间，指到格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)的总毫秒数
    pub batch_end_time: i64,
    /// 优惠金额	单位: 分
    pub discount: i64,
    /// 可领取数量
    pub init_quantity: i64,
    /// 每个用户限领张数
    pub user_limit: i64,
    /// 商品ID
    pub goods_id: i64,
}

impl RequestType for PddPromotionGoodsCouponCreateRequest {
    type Response = PddPromotionGoodsCouponCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.goods.coupon.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionGoodsCouponCreateResponse {
    /// 创建的无门槛商品劵批次id
    #[serde(default)]
    pub batch_id: i64,
}
