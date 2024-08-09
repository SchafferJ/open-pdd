//! # 根据尺码表模板id查询自定义尺码表模版
//!
//! 管理尺码表模板时需要单独查询尺码表模板
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSizespecTemplateGetRequest {
    /// 尺码表id
    pub id: i64,
}

impl RequestType for PddGoodsSizespecTemplateGetRequest {
    type Response = PddGoodsSizespecTemplateGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.sizespec.template.get"
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
pub struct Meta {
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
pub struct Records {
    /// 尺码组和尺码表元素的值
    #[serde(default)]
    pub values: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    /// 尺码表元数据（表头）
    #[serde(default)]
    pub meta: Option<Meta>,
    /// 尺码表图片
    #[serde(default)]
    pub pic_url: String,
    /// 尺码表行数据
    #[serde(default)]
    pub records: Option<Vec<Records>>,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 尺码表分类id
    #[serde(default)]
    pub class_id: i64,
    /// 尺码表内容
    #[serde(default)]
    pub content: Option<Content>,
    /// 创建时间
    #[serde(default)]
    pub create_at: i64,
    /// 扩展类型
    #[serde(default)]
    pub extend_type: i32,
    /// 尺码表id
    #[serde(default)]
    pub id: i64,
    /// 是否尺码表分类默认
    #[serde(default)]
    pub is_default: bool,
    /// 尺码表属于的店铺
    #[serde(default)]
    pub mall_id: i64,
    /// 尺码表名称
    #[serde(default)]
    pub name: String,
    /// 是否可复用
    #[serde(default)]
    pub reusable: bool,
    /// 更新时间
    #[serde(default)]
    pub update_at: i64,
    /// 使用的商品数
    #[serde(default)]
    pub used: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSizespecTemplateGetResponse {
    /// 错误码
    #[serde(default)]
    pub error_code: i32,
    /// 错误描述
    #[serde(default)]
    pub error_msg: String,
    /// 结果
    #[serde(default)]
    pub result: Option<Result>,
    /// 请求成功
    #[serde(default)]
    pub success: bool,
}
