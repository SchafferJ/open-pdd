//! # 删除自定义尺码表模版
//!
//! 管理尺码表模板时需要删除自定义尺码表模板
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSizespecTemplateDeleteRequest {
    /// 尺码表模板id
    pub id: i64,
}

impl RequestType for PddGoodsSizespecTemplateDeleteRequest {
    type Response = PddGoodsSizespecTemplateDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.sizespec.template.delete"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSizespecTemplateDeleteResponse {
    /// 错误码
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    /// 错误描述
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 删除成功
    #[serde(default)]
    pub result: bool,
    /// 请求成功
    #[serde(default)]
    pub success: bool,
}
