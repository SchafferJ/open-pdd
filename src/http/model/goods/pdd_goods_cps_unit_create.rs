//! # 设置单品推广API
//!
//! 批量设置商品推广API，其中创建或修改优惠券均应用于该商品所有生效的推广计划
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Units {
    /// 商品id
    pub goods_id: i64,
    /// 佣金比例（千分比）
    pub rate: i64,
    /// 优惠券id
    pub coupon_id: Option<i64>,
    /// 优惠券开始时间，商品售价>=10元必传
    pub coupon_start_time: Option<String>,
    /// 优惠券结束时间，商品售价>=10元必传
    pub coupon_end_time: Option<String>,
    /// 优惠券面额（单位为分），商品售价>=10元必传
    pub discount: Option<i64>,
    /// 设置的优惠券张数，商品售价>=10元必传
    pub init_quantity: Option<i64>,
    /// 剩余的优惠券张数，商品售价>=10元必传
    pub remain_quantity: Option<i64>,
    /// 优惠券领取后的有效使用时间天数
    pub duration: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsCpsUnitCreateRequest {
    /// 批量设置商品字符串，按照此格式来填写，具体参数见下。注意：创建优惠券时，必须优惠券几个参数都填，否则商品能创建成功,但是默认没有优惠券：[{"goods_id":10000,"rate":2,"coupon_id":123456,"coupon_start_time":1572345,"coupon_end_time":1576482,"discount":300,"init_quantity":600,"remain_quantity":560,"duration":30},{...}........]
    pub units: Vec<Units>,
}

impl RequestType for PddGoodsCpsUnitCreateRequest {
    type Response = PddGoodsCpsUnitCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cps.unit.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCpsUnitCreateResponse {
    /// 创建失败的商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 创建失败原因
    #[serde(default)]
    pub reason: String,
}
