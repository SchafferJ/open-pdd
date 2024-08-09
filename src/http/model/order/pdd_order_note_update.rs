//! # 编辑商家订单备注
//!
//! 编辑商家订单备注信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderNoteUpdateRequest {
    /// 订单备注
    pub note: String,
    /// 备注标记：1-红色，2-黄色，3-绿色，4-蓝色，5-紫色，tag与tag_name关联，都入参或都不入参
    pub tag: Option<i32>,
    /// 标记名称；长度最大为3个字符，tag与tag_name关联，都入参或都不入参
    pub tag_name: Option<String>,
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddOrderNoteUpdateRequest {
    type Response = PddOrderNoteUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.note.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOrderNoteUpdateResponse {
    /// 是否请求成功
    #[serde(default)]
    pub success: bool,
    /// 错误码
    #[serde(default)]
    pub error_code: i32,
    /// 错误信息
    #[serde(default)]
    pub error_msg: String,
}
