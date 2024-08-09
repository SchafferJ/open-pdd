//! # 虚拟业务信息查询接口
//!
//! 该接口用于查询虚拟业务订单的特有字段
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderVirtualInformationGetRequest {
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddOrderVirtualInformationGetRequest {
    type Response = PddOrderVirtualInformationGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.virtual.information.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOrderVirtualInformationGetResponse {
    /// 联系人手机号
    #[serde(default)]
    pub contact_phone: String,
    /// 身份证名
    #[serde(default)]
    pub id_card_name: String,
    /// 身份证号
    #[serde(default)]
    pub id_card_num: String,
    /// 选号卡号
    #[serde(default)]
    pub phone_number_chosen_by_user: String,
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
}
