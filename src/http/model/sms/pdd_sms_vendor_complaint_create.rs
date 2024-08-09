//! # 投诉号码上传
//!
//! 短信供应商投诉号码上传
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddSmsVendorComplaintCreateRequest {
    /// cmpp账号名
    pub account: String,
    /// 投诉时间(格式yyyy-MM-dd HH:mm:ss)
    pub complaint_time: String,
    /// 投诉次数
    pub count: i32,
    /// 短信下发时间(格式yyyy-MM-dd HH:mm:ss)
    pub deliver_time: Option<String>,
    /// 手机号码
    pub mobile: String,
    /// 归属运营商
    pub operator: String,
    /// 归属地省份
    pub province: String,
    /// 短信投诉内容(不超过500个字)
    pub sms_content: Option<String>,
}

impl RequestType for PddSmsVendorComplaintCreateRequest {
    type Response = PddSmsVendorComplaintCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.sms.vendor.complaint.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddSmsVendorComplaintCreateResponse {
    #[serde(default)]
    pub error_code: i32,
    #[serde(default)]
    pub error_msg: String,
    #[serde(default)]
    pub success: bool,
}
