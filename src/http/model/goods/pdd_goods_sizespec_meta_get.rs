//! # 尺码组和尺码参数查询
//!
//! 创建尺码表需要查询尺码表分类支持的尺码组和尺码参数（元数据）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSizespecMetaGetRequest {
    /// 尺码分类id
    pub class_id: i32,
}

impl RequestType for PddGoodsSizespecMetaGetRequest {
    type Response = PddGoodsSizespecMetaGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.sizespec.meta.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Elements {
    /// 尺码元数据id
    #[serde(default)]
    pub id: i32,
    /// 尺码元数据名称
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Groups {
    /// 尺码元数据id
    #[serde(default)]
    pub id: i32,
    /// 尺码元数据名称
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 可批量的尺码元素id
    #[serde(default)]
    pub batchable_element_ids: Vec<i32>,
    /// 尺码元素
    #[serde(default)]
    pub elements: Option<Vec<Elements>>,
    /// 尺码组
    #[serde(default)]
    pub groups: Option<Vec<Groups>>,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSizespecMetaGetResponse {
    /// 错误码
    #[serde(default)]
    pub error_code: i32,
    /// 错误消息
    #[serde(default)]
    pub error_msg: String,
    /// 结果
    #[serde(default)]
    pub result: Option<Result>,
    /// 请求成功
    #[serde(default)]
    pub success: bool,
}
