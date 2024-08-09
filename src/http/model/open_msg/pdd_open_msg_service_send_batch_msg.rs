//! # 短信批量发送接口
//!
//! 短信批量发送接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOpenMsgServiceSendBatchMsgRequest {
    /// 业务请求唯一标识
    pub out_id: Option<String>,
    /// 接收短信的手机号码列表（仅允许密文）,["密文1", "密文2"]
    pub phone_numbers: Vec<String>,
    /// 短信签名名称
    pub sign_name: String,
    /// 上行短信扩展码
    pub sms_up_extend_code: Option<String>,
    /// 短信模板CODE
    pub template_code: i64,
    /// 短信模板变量JSON集合(与手机号对应),示例："${param}","aaa"，注意${}符号勿遗漏
    pub template_param_json: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
}

impl RequestType for PddOpenMsgServiceSendBatchMsgRequest {
    type Response = PddOpenMsgServiceSendBatchMsgResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.msg.service.send.batch.msg"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOpenMsgServiceSendBatchMsgResponse;
