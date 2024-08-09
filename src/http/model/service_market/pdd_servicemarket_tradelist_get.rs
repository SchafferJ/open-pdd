//! # 交易明细单导出
//!
//! 用于ISV查询自己名下的服务的交易明细单
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddServicemarketTradelistGetRequest {
    /// 查询起始时间，精确到秒，起止时间间隔最大31天
    pub begin_time: i32,
    /// 查询结束时间，精确到秒，起止时间间隔最大31天
    pub end_time: i32,
    /// 收支类型，空-全部 1-收入 2-支出
    pub group_type: Option<i32>,
    /// 分页页码，最大1000
    pub page: i32,
    /// 分页大小，最大100
    pub page_size: i32,
    /// 服务订单号
    pub service_order_sn: Option<String>,
}

impl RequestType for PddServicemarketTradelistGetRequest {
    type Response = PddServicemarketTradelistGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.servicemarket.tradelist.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// 交易金额，单位分
    #[serde(default)]
    pub amount: i64,
    /// 流水类目名称，1-交易收入，2-优惠券结算，3-退款，4-提现，5-技术服务费，7-扣款
    #[serde(default)]
    pub class_name: String,
    /// 流水创建时间，单位秒
    #[serde(default)]
    pub created_at: i32,
    /// 资金流向，1-收入，2-支出
    #[serde(default)]
    pub group_type: i32,
    /// 服务订单号
    #[serde(default)]
    pub service_order_sn: String,
    /// 交易备注
    #[serde(default)]
    pub trade_note: String,
}

#[derive(Debug, Deserialize)]
pub struct PddServicemarketTradelistGetResponse {
    /// 交易流水列表
    #[serde(default)]
    pub data: Option<Vec<Data>>,
    /// 当前查询条件下所有订单总数
    #[serde(default)]
    pub total_count: i32,
}
