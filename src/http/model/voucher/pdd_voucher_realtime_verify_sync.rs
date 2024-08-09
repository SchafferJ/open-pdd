//! # 卡券API核销券码
//!
//! 用户使用券码时，商家需要实时给PDD侧回传券码核销结果
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 订单号
    pub order_sn: String,
    /// 备注
    pub remark: Option<String>,
    /// ISV核销流水号
    pub serial_no: String,
    /// 门店名称
    pub shop_name: Option<String>,
    /// 门店编号
    pub shop_no: Option<String>,
    /// 卡券核销时间（13 位毫秒）
    pub verify_time: i64,
    /// 卡券编号
    pub out_voucher_id: String,
}

#[derive(Debug, Serialize)]
pub struct PddVoucherRealtimeVerifySyncRequest {
    /// 请求入参
    pub request: Request,
}

impl RequestType for PddVoucherRealtimeVerifySyncRequest {
    type Response = PddVoucherRealtimeVerifySyncResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.realtime.verify.sync"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherRealtimeVerifySyncResponse;
