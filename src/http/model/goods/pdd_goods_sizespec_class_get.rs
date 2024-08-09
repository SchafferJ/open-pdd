//! # 尺码表分类查询
//!
//! 管理尺码表模板、创建尺码表模板需要使用分类管理
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSizespecClassGetRequest;

impl RequestType for PddGoodsSizespecClassGetRequest {
    type Response = PddGoodsSizespecClassGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.sizespec.class.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Children {
    /// 尺码表分类id
    #[serde(default)]
    pub id: i32,
    /// 尺码表分类名称
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 尺码表叶分类（二级分类）
    #[serde(default)]
    pub children: Option<Vec<Children>>,
    /// 尺码表分类id
    #[serde(default)]
    pub id: i32,
    /// 尺码表分类名称
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSizespecClassGetResponse {
    /// 错误码
    #[serde(default)]
    pub error_code: i32,
    /// 错误描述
    #[serde(default)]
    pub error_msg: String,
    /// 结果
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    /// 请求成功
    #[serde(default)]
    pub success: bool,
}
