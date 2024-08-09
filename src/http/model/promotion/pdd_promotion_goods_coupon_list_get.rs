//! # 商品优惠券批次列表查询
//!
//! 商品优惠券批次列表查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionGoodsCouponListGetRequest {
    /// 页码，默认1
    pub page: Option<i32>,
    /// 每页数量，默认100
    pub page_size: Option<i32>,
    /// 商品ID
    pub goods_id: Option<i64>,
    /// 查询范围	0 全部，1 多多进宝券，2 无门槛商品券；默认1
    pub query_range: Option<i32>,
    /// 批次状态	1 领取中，2 已领完，3 已结束，4 已暂停
    pub batch_status: Option<i32>,
    /// 排序	1 创建时间正序，2 创建时间倒序，3 开始时间正序，4 开始时间倒序，5 初始数量正序， 6 初始数量倒序，7 领取数量正序，8 领取数量倒序；默认2
    pub sort_by: Option<i32>,
}

impl RequestType for PddPromotionGoodsCouponListGetRequest {
    type Response = PddPromotionGoodsCouponListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.goods.coupon.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct CouponBatch {
    /// 批次ID
    #[serde(default)]
    pub id: i64,
    /// 批次描述
    #[serde(default)]
    pub batch_desc: String,
    /// 折扣参数，为请求中传入的discountAmount，表示折扣金额，单位: 分
    #[serde(default)]
    pub discount_param: i64,
    /// 最小可用订单金额
    #[serde(default)]
    pub min_order_amount: i64,
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
    /// 批次开始时间
    #[serde(default)]
    pub batch_start_time: i64,
    /// 批次结束时间
    #[serde(default)]
    pub batch_end_time: i64,
    /// 券来源类型，45 店铺多多进宝商品券，54 无门槛商品券，87 店铺多多进宝大淘客定向商品券，88 店铺多多果园商品券
    #[serde(default)]
    pub source_type: i32,
    /// 商品ID
    #[serde(default)]
    pub goods_id: i64,
    /// 商品名
    #[serde(default)]
    pub goods_name: String,
    /// 图片URL
    #[serde(default)]
    pub image_url: String,
    /// 批次状态，1 领取中，2 已领完，3 已结束，4 已暂停
    #[serde(default)]
    pub status: i32,
    /// 批次创建时间
    #[serde(default)]
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionGoodsCouponListGetResponse {
    /// 返回商品优惠券对象
    #[serde(default)]
    pub coupon_batch_list: Option<Vec<CouponBatch>>,
    /// 返回的优惠券总数
    #[serde(default)]
    pub total_size: i32,
}
