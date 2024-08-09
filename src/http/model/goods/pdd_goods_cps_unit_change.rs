//! # 修改商品推广API
//!
//! 修改推广商品API
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCpsUnitChangeRequest {
    /// 优惠券结束时间
    pub coupon_end_time: Option<String>,
    /// 优惠券id
    pub coupon_id: Option<i64>,
    /// 优惠券号
    pub coupon_sn: Option<String>,
    /// 优惠券开始时间
    pub coupon_start_time: Option<String>,
    /// 优惠券面额（单位为分）
    pub discount: Option<i32>,
    /// 商品id
    pub goods_id: i64,
    /// 设置的优惠券张数
    pub init_quantity: Option<i64>,
    /// 佣金比例（千分比）
    pub rate: i32,
    /// 优惠券剩余数量
    pub remain_quantity: Option<i64>,
    /// 优惠券领取后的有效使用时间天数
    pub duration: Option<i32>,
}

impl RequestType for PddGoodsCpsUnitChangeRequest {
    type Response = PddGoodsCpsUnitChangeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cps.unit.change"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCpsUnitChangeResponse;
