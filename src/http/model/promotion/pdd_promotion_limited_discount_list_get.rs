//! # 限时限量购活动列表查询
//!
//! 查询已创建的限时限量购活动列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPromotionLimitedDiscountListGetRequest {
    /// 支持多个活动类型的查询。3-限量折扣；12-限时折扣。
    pub activity_types: Vec<i32>,
    /// 商品id列表
    pub goods_id_list: Option<Vec<i64>>,
    /// 默认false。true-仅返回活动数量；false-返回活动数量和活动设置数据
    pub just_count: Option<bool>,
    /// 0:创建时间升序  1:创建时间降序
    pub order: Option<i32>,
    /// 页码，默认1
    pub page_no: Option<i32>,
    /// 页大小，默认10
    pub page_size: Option<i32>,
    /// 支持多个活动状态的查询。1-未开始，2-进行中，3-结束|系统结束，4-结束|商家结束，5-提前结束，6-商品处罚中（屏蔽中）。其中3,4,5都算是结束状态。
    pub status_list: Vec<i32>,
}

impl RequestType for PddPromotionLimitedDiscountListGetRequest {
    type Response = PddPromotionLimitedDiscountListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.promotion.limited.discount.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPromotionLimitedDiscountListGetResponse;
