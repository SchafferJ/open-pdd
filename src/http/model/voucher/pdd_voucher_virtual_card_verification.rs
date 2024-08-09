//! # 卡券（电子）核销接口
//!
//! 卡券（电子）核销接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct VoucherData {
    /// 流水号
    pub out_trans_no: String,
    /// 券状态更改时间
    pub voucher_time: i64,
    /// 券状态 1：已核销；2：已销毁
    pub voucher_status: i32,
    /// 券号
    pub voucher_no: String,
}

#[derive(Debug, Serialize)]
pub struct PddVoucherVirtualCardVerificationRequest {
    /// 拼多多订单号
    pub order_sn: String,
    /// 券信息列表
    pub voucher_data_list: Vec<VoucherData>,
}

impl RequestType for PddVoucherVirtualCardVerificationRequest {
    type Response = PddVoucherVirtualCardVerificationResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.virtual.card.verification"
    }
}

#[derive(Debug, Deserialize)]
pub struct Voucher {
    /// 卡号
    #[serde(default)]
    pub card_no: String,
    /// 卡密
    #[serde(default)]
    pub card_code: String,
    /// 标准密码
    #[serde(default)]
    pub mark_password: String,
    /// 状态
    #[serde(default)]
    pub status: i32,
    /// 返回状态
    #[serde(default)]
    pub refund_status: i32,
    /// 验证处
    #[serde(default)]
    pub verification_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherVirtualCardVerificationResponse {
    /// 拼多多订单号
    #[serde(default)]
    pub order_sn: String,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 商品属性id
    #[serde(default)]
    pub sku_id: i64,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 券信息数组
    #[serde(default)]
    pub voucher_list: Option<Vec<Voucher>>,
}
