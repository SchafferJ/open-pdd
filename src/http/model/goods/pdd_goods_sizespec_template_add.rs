//! # 新增自定义尺码表模版
//!
//! 管理尺码表模板时需要新增自定义尺码表模版
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Elements {
    /// 尺码元数据id
    pub id: i32,
    /// 尺码元数据名称
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Groups {
    /// 尺码元数据id
    pub id: i32,
    /// 尺码元数据名称
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Meta {
    /// 尺码元素
    pub elements: Vec<Elements>,
    /// 尺码组
    pub groups: Vec<Groups>,
}

#[derive(Debug, Serialize)]
pub struct Records {
    /// 尺码组和尺码表元素的值
    pub values: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct Content {
    /// 尺码表元数据（表头），pdd.goods.sizespec.meta.get得到
    pub meta: Meta,
    /// 尺码表行数据
    pub records: Vec<Records>,
}

#[derive(Debug, Serialize)]
pub struct SizeSpecDto {
    /// 尺码表分类id，pdd.goods.sizespec.class.get得到
    pub class_id: i32,
    /// 尺码表内容
    pub content: Content,
    /// 尺码表名称
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsSizespecTemplateAddRequest {
    /// 尺码表
    pub size_spec_dto: SizeSpecDto,
}

impl RequestType for PddGoodsSizespecTemplateAddRequest {
    type Response = PddGoodsSizespecTemplateAddResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.sizespec.template.add"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSizespecTemplateAddResponse {
    /// 错误码
    #[serde(default)]
    pub error_code: i32,
    /// 错误描述
    #[serde(default)]
    pub error_msg: String,
    /// 新建尺码表id
    #[serde(default)]
    pub result: i64,
    /// 请求成功
    #[serde(default)]
    pub success: bool,
}
