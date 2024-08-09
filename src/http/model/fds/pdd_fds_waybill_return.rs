//! # 电子面单回传
//!
//! 厂家回传电子面单到商家订单
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ParamFdsWaybillReturnRequest {
    /// 代打店铺id
    pub mall_mask_id: String,
    /// 代打订单号
    pub order_mask_sn: String,
    /// 面单号
    pub waybill_code: String,
    /// 物流公司 Code ，枚举： YTO- 圆通，ZTO-中通，YUNDA-韵达，STO-申通
    pub wp_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddFdsWaybillReturnRequest {
    /// 入参信息
    pub param_fds_waybill_return_request: ParamFdsWaybillReturnRequest,
}

impl RequestType for PddFdsWaybillReturnRequest {
    type Response = PddFdsWaybillReturnResponse;

    fn get_type(&self) -> &'static str {
        "pdd.fds.waybill.return"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddFdsWaybillReturnResponse {
    /// 回传结果true:成功false：失败
    #[serde(default)]
    pub return_result: bool,
}
