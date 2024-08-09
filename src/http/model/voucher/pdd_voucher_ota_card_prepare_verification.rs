//! # 平台卡密核销验券
//!
//! 查询平台生成卡密对应的卡券信息、商品信息和订单信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 卡密
    pub card_no: String,
    /// 门店id
    pub store_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddVoucherOtaCardPrepareVerificationRequest {
    /// 请求体
    pub request: Request,
}

impl RequestType for PddVoucherOtaCardPrepareVerificationRequest {
    type Response = PddVoucherOtaCardPrepareVerificationResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.ota.card.prepare.verification"
    }
}

#[derive(Debug, Deserialize)]
pub struct CardVo {
    /// 卡券有效期结束时间，单位秒
    #[serde(default)]
    pub available_end_time: i64,
    /// 卡券有效期开始时间，单位秒
    #[serde(default)]
    pub available_start_time: i64,
    /// 打码卡密
    #[serde(default)]
    pub mask_card_no: String,
    /// 剩余可用次数
    #[serde(default)]
    pub remain_times: i32,
    /// 券状态码。1-未核销，2-已核销， 3-已过期，4-已销毁
    #[serde(default)]
    pub status: i32,
    /// 状态文案
    #[serde(default)]
    pub status_tips: String,
    /// 总次数
    #[serde(default)]
    pub total_times: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrderGoodsVo {
    /// 商品标题
    #[serde(default)]
    pub goods_name: String,
    /// 购买商品数
    #[serde(default)]
    pub goods_number: i32,
    /// 外部商品编码
    #[serde(default)]
    pub out_goods_sn: String,
    /// 外部sku编码
    #[serde(default)]
    pub out_sku_sn: String,
    /// 规格
    #[serde(default)]
    pub spec: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderVo {
    /// 用户实付金额
    #[serde(default)]
    pub order_amount_fen: i64,
    /// 订单编号
    #[serde(default)]
    pub order_sn: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 卡券信息
    #[serde(default)]
    pub card_vo: Option<CardVo>,
    /// 商品信息
    #[serde(default)]
    pub order_goods_vo: Option<OrderGoodsVo>,
    /// 订单信息
    #[serde(default)]
    pub order_vo: Option<OrderVo>,
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherOtaCardPrepareVerificationResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 卡密和订单信息
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
