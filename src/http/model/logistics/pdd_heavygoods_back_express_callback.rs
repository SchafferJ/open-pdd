//! # 重抛逆向运单回传
//!
//! 重抛直邮订单，如果产生逆向物流轨迹，供应商回传逆向运单信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct PddHeavygoodsBackExpressCallbackRequest {
    /// 包裹回退原因
    pub back_reason: Option<String>,
    /// 逆向运单号
    pub back_trck_no: String,
    /// 逆向运单所关联的正向运单号
    pub trck_no: String,
}

impl RequestType for PddHeavygoodsBackExpressCallbackRequest {
    type Response = PddHeavygoodsBackExpressCallbackResponse;

    fn get_type(&self) -> &'static str {
        "pdd.heavygoods.back.express.callback"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddHeavygoodsBackExpressCallbackResponse;
