//! # 订单检索接口
//!
//! 根据收件人信息检索订单号（只能获取到下单时间三个月以内的交易信息，入参的收件人姓名和收件人电话号码至少有一个值不为空）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderSearchOrderRequest {
    /// 必填，下单时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数，开始时间结束时间间距不超过90天
    pub end_created_at: i64,
    /// 收件人姓名
    pub receiver_name: Option<String>,
    /// 收件人电话号码
    pub receiver_phone: Option<String>,
    /// 必填，下单时间开始时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub start_created_at: i64,
}

impl RequestType for PddOrderSearchOrderRequest {
    type Response = PddOrderSearchOrderResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.search.order"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOrderSearchOrderResponse {
    /// 检索出的订单号列表。按照下单时间倒序，最多返回最近的100笔订单
    #[serde(default)]
    pub order_sn_list: Vec<String>,
}
