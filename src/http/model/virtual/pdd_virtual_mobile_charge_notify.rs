//! # 虚拟类目发货通知接口
//!
//! 虚拟类目发货的接口【仅供话费/流量直冲商家自研对接进行话费流量发货使用】
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ChargeCerti {
    /// 直充充值金额（单位：分）
    pub charge_certi_amount: Option<i64>,
    /// 直充充值成功时间 （yyyy-MM-dd HH:mm:ss格式）
    pub charge_certi_date: Option<String>,
    /// 直充充值号码
    pub charge_certi_mobile: Option<String>,
    /// 充值卡号尾号
    pub charge_certi_mobile_tail: Option<String>,
    /// 直充充值单号
    pub charge_certi_order_sn: Option<String>,
    /// 直充短信原文
    pub charge_certi_text: Option<String>,
    /// 代理商(渠道)编号
    pub merchant_outer_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PddVirtualMobileChargeNotifyRequest {
    /// 直充附加信息对象数组
    pub charge_certi: Option<Vec<ChargeCerti>>,
    /// 电子发票信息 0-不支持开具  1-支持开具
    pub ele_invoice: Option<i32>,
    /// 拼多多订单编码
    pub order_sn: String,
    /// 11122dafa 外部系统订单编码
    pub outer_order_sn: String,
    /// 虚拟系统充值结果，SUCCESS-充值成功，FAIL-充值失败
    pub status: String,
}

impl RequestType for PddVirtualMobileChargeNotifyRequest {
    type Response = PddVirtualMobileChargeNotifyResponse;

    fn get_type(&self) -> &'static str {
        "pdd.virtual.mobile.charge.notify"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddVirtualMobileChargeNotifyResponse {
    /// 回调通知结果，true-成功，false-失败
    #[serde(default)]
    pub success: bool,
}
