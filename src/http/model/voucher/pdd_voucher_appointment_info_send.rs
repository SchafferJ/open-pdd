//! # 卡券预约提货接口
//!
//! 第三方ISV将消费者的预约提货信息同步给平台
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
pub struct PddVoucherAppointmentInfoSendRequest {
    /// 订单号
    pub order_sn: String,
    /// 外部流水号
    pub out_biz_no: String,
    /// 优惠券信息列表,例子[{"voucher_id":"test voucher_id","voucher_no":"test voucher_no"}]
    pub voucher_list: Vec<Voucher>,
    /// 物流方式  1  物流发货   2 自提
    pub logistics_type: i32,
    /// 预约时间, 距离格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总毫秒数
    pub appointment_time: i64,
}

impl RequestType for PddVoucherAppointmentInfoSendRequest {
    type Response = PddVoucherAppointmentInfoSendResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.appointment.info.send"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherAppointmentInfoSendResponse {
    /// is_success
    #[serde(default)]
    pub is_success: bool,
}
