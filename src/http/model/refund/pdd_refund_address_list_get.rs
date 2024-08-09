//! # 获取商家退货地址库
//!
//! 获取商家退货地址库
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddRefundAddressListGetRequest;

impl RequestType for PddRefundAddressListGetRequest {
    type Response = PddRefundAddressListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.refund.address.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct RefundAddress {
    /// 退货地址所在城市ID
    #[serde(default)]
    pub city_id: i32,
    /// 退货地址所在城市名字
    #[serde(default)]
    pub city_name: String,
    /// 退货地址所在区ID
    #[serde(default)]
    pub district_id: i32,
    /// 退货地址所在区名字
    #[serde(default)]
    pub district_name: String,
    /// 退货地址ID
    #[serde(default)]
    pub id: i64,
    /// 是否为默认退货地址
    #[serde(default)]
    pub is_default: String,
    /// 退货地址是否合法
    #[serde(default)]
    pub is_legal: bool,
    /// 退货地址是否有效
    #[serde(default)]
    pub is_validated: bool,
    /// 店铺ID
    #[serde(default)]
    pub mall_id: i64,
    /// 退货地址所在省份ID
    #[serde(default)]
    pub province_id: i32,
    /// 退货地址所在省份名字
    #[serde(default)]
    pub province_name: String,
    /// 退货地址
    #[serde(default)]
    pub refund_address: String,
    /// refund_id
    #[serde(default)]
    pub refund_address_id: String,
    /// 退货收件人名字
    #[serde(default)]
    pub refund_name: String,
    /// 退货收件人手机号
    #[serde(default)]
    pub refund_phone: String,
    /// 退货收件人固定电话
    #[serde(default)]
    pub refund_tel: String,
}

#[derive(Debug, Deserialize)]
pub struct PddRefundAddressListGetResponse {
    /// 退货地址列表
    #[serde(default)]
    pub refund_address_list: Option<Vec<RefundAddress>>,
}
