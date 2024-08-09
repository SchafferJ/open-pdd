//! # 卡券信息发送接口
//!
//! 第三方ISV将消费者购买的卡券信息同步给平台
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Voucher {
    /// 卡券ID
    pub voucher_id: String,
    /// 卡券号
    pub voucher_no: String,
}

#[derive(Debug, Serialize)]
pub struct PddVoucherVoucherInfoSendRequest {
    /// 订单号
    pub order_sn: String,
    /// 外部流水号
    pub out_biz_no: String,
    /// 卡券信息列表,例子[{"voucher_id":"test voucher_id","voucher_no":"test voucher_no"}]
    pub voucher_list: Vec<Voucher>,
}

impl RequestType for PddVoucherVoucherInfoSendRequest {
    type Response = PddVoucherVoucherInfoSendResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.voucher.info.send"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherVoucherInfoSendResponse {
    /// 是否请求成功
    #[serde(default)]
    pub is_success: bool,
}
