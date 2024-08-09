//! # 寄件核重信息回告
//!
//! 三方快递服务商回传寄件单的内网重量
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct Request {
    /// 物流环节首发或到达省中心称重，单位克
    #[serde(rename = "centerWeight")]
    pub center_weight: Option<i32>,
    /// 结算重量，单位克
    #[serde(rename = "checkWeight")]
    pub check_weight: i32,
    /// 快递公司编码
    #[serde(rename = "expressCode")]
    pub express_code: String,
    /// 是否使用抛重
    #[serde(rename = "isUseVolumetric")]
    pub is_use_volumetric: bool,
    /// 运单号
    #[serde(rename = "mailNo")]
    pub mail_no: String,
    /// 抛重体积
    pub volume: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PddServiceMailCheckWeightInfoRequest {
    /// 请求参数
    pub request: Request,
}

impl RequestType for PddServiceMailCheckWeightInfoRequest {
    type Response = PddServiceMailCheckWeightInfoResponse;

    fn get_type(&self) -> &'static str {
        "pdd.service.mail.check.weight.info"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddServiceMailCheckWeightInfoResponse {
    /// 错误代码
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    /// 错误信息
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 同步结果
    #[serde(default)]
    pub flag: bool,
}
