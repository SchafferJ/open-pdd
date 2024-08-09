//! # 线上服务市场订单查询接口
//!
//! 用于拉取回流完成的订单以及线上增量的订购订单
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddVasOrderSearchRequest {
    /// 创建时间结束，UNIX时间戳（ms 级别），默认为当前时间，支持最大范围为7天。
    pub create_time_end: Option<i64>,
    /// 创建时间开始，UNIX时间戳（ms级别），默认为当前时间往前推7天，支持最大范围为7天。
    pub create_time_start: Option<i64>,
    /// 买家店铺id
    pub mall_id: Option<i64>,
    /// 服务订单号
    pub order_sn: Option<String>,
    /// 订单状态，枚举值，0-未完成，1-已完成，2-已取消，空-全部
    pub order_status: Option<i32>,
    /// 分页页码
    pub page: i32,
    /// 分页大小，最大支持100
    pub page_size: i32,
    /// 支付时间开始，UNIX时间戳（ms 级别）
    pub pay_time_end: Option<i64>,
    /// 支付时间开始，UNIX时间戳（ms 级别）
    pub pay_time_start: Option<i64>,
    /// 服务sku_id，可在服务详情页中获取
    pub sku_id: Option<i64>,
    /// 售后状态，0-未售后，1-已售后
    pub refund_status: Option<i32>,
}

impl RequestType for PddVasOrderSearchRequest {
    type Response = PddVasOrderSearchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.vas.order.search"
    }
}

#[derive(Debug, Deserialize)]
pub struct Orders {
    /// 实付价格
    #[serde(default)]
    pub amount: i64,
    /// 订单创建时间
    #[serde(default)]
    pub create_time: i64,
    /// 店铺ID
    #[serde(default)]
    pub mall_id: i64,
    /// 服务订单ID
    #[serde(default)]
    pub order_sn: String,
    /// 支付状态，枚举值，0-未支付，1-已支付
    #[serde(default)]
    pub pay_status: i32,
    /// 支付时间
    #[serde(default)]
    pub pay_time: i64,
    /// 服务ID
    #[serde(default)]
    pub service_id: i64,
    /// 服务名称
    #[serde(default)]
    pub service_name: String,
    /// 服务SKUID
    #[serde(default)]
    pub sku_id: i64,
    /// 服务SKU名称
    #[serde(default)]
    pub sku_spec: String,
    /// 订单来源，0-线上订购，1-线下回流
    #[serde(default)]
    pub source: i32,
    /// 订购时长
    #[serde(default)]
    pub time_length: i64,
    /// 售后完成时间，如果没有申请过售后则为null
    #[serde(default)]
    pub refund_finish_time: i64,
    /// 售后状态，0-未售后，1-已售后
    #[serde(default)]
    pub refund_status: i32,
    /// 店铺名称
    #[serde(default)]
    pub mall_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddVasOrderSearchResponse {
    /// 订单列表
    #[serde(default)]
    pub orders: Option<Vec<Orders>>,
    /// 当前查询条件下所有订单总数
    #[serde(default, rename = "totalCount")]
    pub total_count: i32,
}
