//! # 虚拟号查询接口
//!
//! 检查入参号码是否订单所绑定的虚拟号
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOpenVirtualNumberCheckRequest {
    /// 分机号，非必填，4位数字字符
    pub identify_number: Option<String>,
    /// 订单号
    pub order_sn: String,
    /// 虚拟号，11位数字字符
    pub virtual_number: String,
}

impl RequestType for PddOpenVirtualNumberCheckRequest {
    type Response = PddOpenVirtualNumberCheckResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.virtual.number.check"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 虚拟号绑定的时间戳
    #[serde(default)]
    pub bind_timestamp: i64,
    /// 错误码, 0:成功
    #[serde(default)]
    pub error_code: i32,
    /// 错误信息
    #[serde(default)]
    pub error_msg: String,
    /// 是否订单所绑定的虚拟号，true是，false否
    #[serde(default)]
    pub r#match: bool,
    /// 11位入参号码是否是虚拟号
    #[serde(default)]
    pub r#virtual: bool,
}

#[derive(Debug, Deserialize)]
pub struct PddOpenVirtualNumberCheckResponse {
    #[serde(default)]
    pub result: Option<Result>,
}
