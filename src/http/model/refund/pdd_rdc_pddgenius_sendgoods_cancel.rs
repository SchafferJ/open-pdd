//! # 取消发货
//!
//! 用于未发货仅退款服务商通知拼多多PG取消成功
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Param {
    /// 错误码：1001 错误场景：该订单未同步，无法取消发货 实际含义：订单未同步到isv; 错误码：1002 错误场景：该订单已发货，无法取消发货 实际含义：订单已经发货; 错误码：1003 错误场景：该订单已发货，无法取消发货 实际含义：订单已打印电子面单
    pub fail_reason_code: Option<i32>,
    /// 描述
    pub msg: Option<String>,
    /// 操作时间戳（毫秒）
    pub operate_time: Option<i64>,
    /// 退款金额 单位 分
    pub refund_fee: Option<i32>,
    /// 退款单ID
    pub refund_id: i64,
    /// 状态SUCCESS、FAIL
    pub status: String,
    /// 订单号
    pub tid: String,
}

#[derive(Debug, Serialize)]
pub struct PddRdcPddgeniusSendgoodsCancelRequest {
    /// param
    pub param: Param,
}

impl RequestType for PddRdcPddgeniusSendgoodsCancelRequest {
    type Response = PddRdcPddgeniusSendgoodsCancelResponse;

    fn get_type(&self) -> &'static str {
        "pdd.rdc.pddgenius.sendgoods.cancel"
    }
}

#[derive(Debug, Deserialize)]
pub struct ResultData {
    /// 退款单ID
    #[serde(default)]
    pub refund_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// result_data
    #[serde(default)]
    pub result_data: Option<ResultData>,
}

#[derive(Debug, Deserialize)]
pub struct PddRdcPddgeniusSendgoodsCancelResponse {
    /// result
    #[serde(default)]
    pub result: Option<Result>,
}
