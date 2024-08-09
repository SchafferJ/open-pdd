//! # 短信发送接口
//!
//! 开平短信服务发送短信接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOpenMsgServiceSendMsgRequest {
    /// 业务请求唯一标识
    pub out_id: Option<String>,
    /// 接收短信的手机号码列表（仅允许密文）,["密文1", "密文2"]
    pub phone_numbers: Vec<String>,
    /// 短信签名名称
    pub sign_name: String,
    /// 上行短信扩展码
    pub sms_up_extend_code: Option<String>,
    /// 短信模板ID
    pub template_code: i64,
    /// 短信模板变量对应的实际值，JSON格式,示例："${param}","aaa"，注意${}符号勿遗漏
    pub template_param: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

impl RequestType for PddOpenMsgServiceSendMsgRequest {
    type Response = PddOpenMsgServiceSendMsgResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.msg.service.send.msg"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOpenMsgServiceSendMsgResponse;
