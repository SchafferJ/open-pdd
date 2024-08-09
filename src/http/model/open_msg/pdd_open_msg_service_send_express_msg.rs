//! # 根据运单号发短信
//!
//! 快递派送过程中根据物流编号发送短信通知
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOpenMsgServiceSendExpressMsgRequest {
    /// 业务请求唯一标识
    pub out_id: Option<String>,
    /// 短信签名名称
    pub sign_name: String,
    /// 上行短信扩展码
    pub sms_up_extend_code: Option<String>,
    /// 短信模板CODE
    pub template_code: i64,
    /// 短信模板变量JSON集合(与手机号对应)与按照手机号发短信一致key变量名 value变量值,示例："${param}","aaa"，注意${}符号勿遗漏
    pub template_param_json: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// 物流单号集合
    pub waybill_codes: Vec<String>,
    /// 快递公司编码
    pub wp_code: String,
}

impl RequestType for PddOpenMsgServiceSendExpressMsgRequest {
    type Response = PddOpenMsgServiceSendExpressMsgResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.msg.service.send.express.msg"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOpenMsgServiceSendExpressMsgResponse;
