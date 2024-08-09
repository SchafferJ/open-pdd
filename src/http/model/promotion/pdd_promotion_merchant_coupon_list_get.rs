//! # 店铺优惠券批次列表接口
//!
//! 店铺优惠券批次列表接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionMerchantCouponListGetRequest {
    /// 页码，默认1
    pub page: Option<i32>,
    /// 每页数量，默认100
    pub page_size: Option<i32>,
    /// 批次开始时间（范围开始）
    pub batch_start_time_from: Option<i64>,
    /// 批次开始时间（范围结束）
    pub batch_start_time_to: Option<i64>,
    /// 批次状态	1 领取中，2 已领完，3 已结束
    pub batch_status: Option<i32>,
    /// 排序	1 创建时间正序，2 创建时间倒序，3 开始时间正序，4 开始时间倒序，5 初始数量正序， 6 初始数量倒序，7 领取数量正序，8 领取数量倒序；默认2
    pub sort_by: Option<i32>,
}

impl RequestType for PddPromotionMerchantCouponListGetRequest {
    type Response = PddPromotionMerchantCouponListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.merchant.coupon.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct CouponBatch {
    /// 批次ID
    #[serde(default)]
    pub id: i64,
    /// 批次名
    #[serde(default)]
    pub batch_name: String,
    /// 批次描述
    #[serde(default)]
    pub batch_desc: String,
    /// 折扣类型，1 代表满减券，2 代表折扣券
    #[serde(default)]
    pub discount_type: i32,
    /// 折扣参数，为请求中传入的discount_amount，表示折扣金额，单位: 分
    #[serde(default)]
    pub discount_param: i64,
    /// 初始数量
    #[serde(default)]
    pub init_quantity: i64,
    /// 剩余数量
    #[serde(default)]
    pub remain_quantity: i64,
    /// 已使用数量
    #[serde(default)]
    pub used_quantity: i64,
    /// 用户限领张数，-1 代表不限制
    #[serde(default)]
    pub user_limit: i64,
    /// 最大折扣金额
    #[serde(default)]
    pub max_discount_amount: i64,
    /// 券有效时长
    #[serde(default)]
    pub duration: i64,
    /// 券有效期时长的单位，0 代表天，2 代表小时
    #[serde(default)]
    pub period_type: i32,
    /// 批次开始时间
    #[serde(default)]
    pub batch_start_time: i64,
    /// 批次结束时间
    #[serde(default)]
    pub batch_end_time: i64,
    /// 券来源类型，16 店铺直接领券，41 店铺精选评价优惠券，66 商家短信营销优惠券
    #[serde(default)]
    pub source_type: i32,
    /// 券类型，固定为8，代表商家券
    #[serde(default)]
    pub r#type: i32,
    /// 批次状态，1 领取中，2 已领完，3 已结束
    #[serde(default)]
    pub status: i32,
    /// 用券条件
    #[serde(default)]
    pub rules: String,
    /// 券展示类型，固定为8，代表商家券
    #[serde(default)]
    pub display_type: i32,
    /// 批次创建时间
    #[serde(default)]
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionMerchantCouponListGetResponse {
    /// 返回店铺优惠券批次数量
    #[serde(default)]
    pub total_size: i32,
    /// 返回店铺优惠券批次对象
    #[serde(default)]
    pub coupon_batch_list: Option<Vec<CouponBatch>>,
}
