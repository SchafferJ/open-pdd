//! # 卡券发货（实物）接口
//!
//! 第三方ISV将商家发货（实物）信息同步给平台
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
pub struct PddVoucherPhysicalGoodsSendRequest {
    /// 订单号
    pub order_sn: String,
    /// 外部流水号
    pub out_biz_no: String,
    /// 优惠券信息列表,例子[{"voucher_id":"test voucher_id","voucher_no":"test voucher_no"}]
    pub voucher_list: Vec<Voucher>,
    /// 物流方式  1  物流发货   2 自提
    pub logistics_type: i32,
    /// 收件人
    pub recipient: String,
    /// 收件人电话
    pub recipient_mobile: String,
    /// 收件人地址
    pub recipient_address: String,
    /// 物流单号
    pub logistics_no: String,
    /// 物流公司编号
    pub logistics_company_id: String,
    /// 物流公司名称
    pub logistics_company: String,
}

impl RequestType for PddVoucherPhysicalGoodsSendRequest {
    type Response = PddVoucherPhysicalGoodsSendResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.physical.goods.send"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherPhysicalGoodsSendResponse {
    /// 请求成功
    #[serde(default)]
    pub is_success: bool,
}
