//! # 同步海淘订单申报失败情况
//!
//! 用于服务商向平台同步海淘订单申报失败和具体原因
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOverseaDeclarationFailNotifyRequest {
    /// 1-超过购买额度，2-清关异常（如重量超标、退运、扣留等）
    pub fail_reason: i32,
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddOverseaDeclarationFailNotifyRequest {
    type Response = PddOverseaDeclarationFailNotifyResponse;

    fn get_type(&self) -> &'static str {
        "pdd.oversea.declaration.fail.notify"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOverseaDeclarationFailNotifyResponse {
    /// true=处理成功
    #[serde(default)]
    pub result: bool,
}
