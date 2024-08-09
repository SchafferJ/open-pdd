//! # 获取所有标准电子面单模板
//!
//! 获取所有标准电子面单模板
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddCloudprintStdtemplatesGetRequest {
    /// 快递公司code
    pub wp_code: Option<String>,
}

impl RequestType for PddCloudprintStdtemplatesGetRequest {
    type Response = PddCloudprintStdtemplatesGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloudprint.stdtemplates.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct StandardTemplates {
    /// 模板id
    #[serde(default)]
    pub standard_template_id: i64,
    /// 模板名称
    #[serde(default)]
    pub standard_template_name: String,
    /// 模板url
    #[serde(default)]
    pub standard_template_url: String,
    /// 模版类型
    #[serde(default)]
    pub standard_waybill_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct Datas {
    /// 该wp的所有标准模板
    #[serde(default)]
    pub standard_templates: Option<Vec<StandardTemplates>>,
    /// wp编码
    #[serde(default)]
    pub wp_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 所有wp的标准模板
    #[serde(default)]
    pub datas: Option<Vec<Datas>>,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudprintStdtemplatesGetResponse {
    /// 结果集
    #[serde(default)]
    pub result: Option<Result>,
}
