//! # 查询短信发送记录
//!
//! 查询短信发送记录
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOpenMsgServiceQueryMsgRecordRequest {
    /// 短信发送流水
    pub biz_id: Option<String>,
    /// 分页参数,页码
    pub page_number: i32,
    /// 分页参数，每页数量。最大值50
    pub page_size: i32,
    /// 查询手机号码
    pub phone_number: String,
    /// 短信发送日期，支持近30天记录查询，格式yyyyMMdd
    pub send_date: String,
}

impl RequestType for PddOpenMsgServiceQueryMsgRecordRequest {
    type Response = PddOpenMsgServiceQueryMsgRecordResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.msg.service.query.msg.record"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOpenMsgServiceQueryMsgRecordResponse;
