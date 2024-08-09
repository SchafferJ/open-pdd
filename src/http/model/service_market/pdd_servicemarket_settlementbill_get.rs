//! # 月结算账单导出
//!
//! 用于ISV查询自己名下的服务的月度结算明细
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddServicemarketSettlementbillGetRequest {
    /// 分页页码，最大不能超过1000
    pub page: i32,
    /// 分页大小，最大不能超过100
    pub page_size: i32,
    /// 服务订单号
    pub service_order_sn: Option<String>,
    /// 结算月份
    pub settle_month: String,
}

impl RequestType for PddServicemarketSettlementbillGetRequest {
    type Response = PddServicemarketSettlementbillGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.servicemarket.settlementbill.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// 服务订单号
    #[serde(default)]
    pub service_order_sn: String,
    /// 当期结算金额，单位分
    #[serde(default)]
    pub settle_amount: i64,
    /// 结算月份
    #[serde(default)]
    pub settle_month: String,
    /// 结算时间
    #[serde(default)]
    pub settle_time: String,
    /// 待结算金额，单位分
    #[serde(default)]
    pub unsettle_amount: i64,
    /// 服务名称
    #[serde(default)]
    pub service_name: String,
    /// 订单类型：0-应用服务、1-客服外包、2-电子发票、3-店铺装修、4-推广托管、5-代运营服务
    #[serde(default)]
    pub order_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddServicemarketSettlementbillGetResponse {
    /// 结算明细列表
    #[serde(default)]
    pub data: Option<Vec<Data>>,
    /// 当前查询条件下所有订单总数
    #[serde(default)]
    pub total_count: i32,
}
