//! # 商家取消获取的电子面单号
//!
//! 商家取消获取的电子面单号
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddWaybillCancelRequest {
    /// 电子面单号
    pub waybill_code: String,
    /// 快递公司code
    pub wp_code: String,
}

impl RequestType for PddWaybillCancelRequest {
    type Response = PddWaybillCancelResponse;

    fn get_type(&self) -> &'static str {
        "pdd.waybill.cancel"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddWaybillCancelResponse {
    /// 调用取消是否成功
    #[serde(default)]
    pub cancel_result: bool,
}
