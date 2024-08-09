//! # 短信明细回执
//!
//! 短信供应商明细回传
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Details {
    /// 短信下发时间(yyyy-MM-dd HH:mm:ss)
    pub deliver_time: String,
    /// 回执状态码，发送成功传DELIVRD
    pub error_code: String,
    /// 短信id,即SubmitResp.msgId,十进制表示
    pub msg_id: i64,
    /// 短信提交时间(yyyy-MM-dd HH:mm:ss)
    pub submit_time: String,
}

#[derive(Debug, Serialize)]
pub struct PddSmsDetailbillPushRequest {
    /// cmpp账号名
    pub account: String,
    /// 批次版本，每天数据必须属于同一个批次，如果重传可以批次号增加，平台以最大批次号为准。一般情况下，批次号固定数字，只有当某天上传数据错误需要弃用时，使用增加后的新批次号。
    pub batch_version: i64,
    /// 数据日期(格式yyyy-MM-dd)
    pub date: String,
    /// 短信明细，detail的列表，list最大100
    pub details: Vec<Details>,
}

impl RequestType for PddSmsDetailbillPushRequest {
    type Response = PddSmsDetailbillPushResponse;

    fn get_type(&self) -> &'static str {
        "pdd.sms.detailbill.push"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddSmsDetailbillPushResponse {
    /// 是否成功
    #[serde(default)]
    pub success: bool,
}
