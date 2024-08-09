//! # 电子面单回传额外运单号
//!
//! 1、代发管理支持厂家回传额外运单号给商家
//! 2、商家可以在MMS后台的代发管理查看额外运单号字段
//! 3、1个订单最多支持上传10个额外运单号，额外的从运单号不能包括主运单号
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 代打店铺id
    pub mall_mask_id: String,
    /// 代打订单号
    pub order_mask_sn: String,
    /// 从运单号列表，最多传递十条从运单号
    pub waybill_codes: Option<Vec<String>>,
    /// 物流公司 Code ，枚举： YTO- 圆通，ZTO-中通，YUNDA-韵达，STO-申通
    pub wp_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddFdsWaybillReturnSlaveRequest {
    /// 回传从运单号请求
    pub request: Request,
}

impl RequestType for PddFdsWaybillReturnSlaveRequest {
    type Response = PddFdsWaybillReturnSlaveResponse;

    fn get_type(&self) -> &'static str {
        "pdd.fds.waybill.return.slave"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddFdsWaybillReturnSlaveResponse {
    /// 回传结果true：成功  false：失败
    #[serde(default)]
    pub result: bool,
}
