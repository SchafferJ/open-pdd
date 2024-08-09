//! # 获取多多国际清关材料
//!
//! 获取多多国际清关材料（按订单维度获取）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOverseaClearanceGetRequest {
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddOverseaClearanceGetRequest {
    type Response = PddOverseaClearanceGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.oversea.clearance.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOverseaClearanceGetResponse {
    /// 身份证姓名
    #[serde(default)]
    pub id_card_name: String,
    /// 身份证号
    #[serde(default)]
    pub id_card_no: String,
    /// 支付申报订单号
    #[serde(default)]
    pub inner_transaction_id: String,
    /// 支付单号
    #[serde(default)]
    pub pay_no: String,
    /// 支付方式，枚举值：WEIXIN,ALIPAY,DUODUOPAY
    #[serde(default)]
    pub pay_type: String,
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
}
