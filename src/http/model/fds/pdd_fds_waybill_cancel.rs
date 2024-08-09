//! # 电子面单取消回传
//!
//! 厂家回传完电子面单，需要删除之前上传的电子面单，可以使用该接口取消回传
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddFdsWaybillCancelInnerRequest {
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
pub struct PddFdsWaybillCancelRequest {
    /// 入参信息
    pub pdd_fds_waybill_cancel_request: PddFdsWaybillCancelInnerRequest,
}

impl RequestType for PddFdsWaybillCancelRequest {
    type Response = PddFdsWaybillCancelResponse;

    fn get_type(&self) -> &'static str {
        "pdd.fds.waybill.cancel"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddFdsWaybillCancelResponse {
    /// 取消回传结果true:成功false：失败
    #[serde(default)]
    pub return_result: bool,
}
