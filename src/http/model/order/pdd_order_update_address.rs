//! # 修改订单收件地址接口
//!
//! 修改订单收件地址 注：风险订单或订单已发货后不可修改
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderUpdateAddressRequest {
    /// 收件详细地址
    pub address: Option<String>,
    /// 收件地城市
    pub city: String,
    /// 城市编码
    pub city_id: i32,
    /// 订单编号
    pub order_sn: String,
    /// 收件地省份
    pub province: String,
    /// 省份编码
    pub province_id: i32,
    /// 收件人姓名
    pub receiver_name: Option<String>,
    /// 收件人电话，明文
    pub receiver_phone: Option<String>,
    /// 收件地区县
    pub town: String,
    /// 区县编码
    pub town_id: i32,
}

impl RequestType for PddOrderUpdateAddressRequest {
    type Response = PddOrderUpdateAddressResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.update.address"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 提示文案
    #[serde(default)]
    pub msg: String,
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
}

#[derive(Debug, Deserialize)]
pub struct PddOrderUpdateAddressResponse {
    /// 请求返回结果
    #[serde(default)]
    pub result: Option<Result>,
    /// 是否请求成功
    #[serde(default)]
    pub success: bool,
    /// 错误码
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    /// 错误信息
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
}
