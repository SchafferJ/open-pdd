//! # 仓库详细信息
//!
//! 仓库详细信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddExpressDepotInfoGetRequest {
    /// 仓库id
    pub depot_id: i64,
}

impl RequestType for PddExpressDepotInfoGetRequest {
    type Response = PddExpressDepotInfoGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.express.depot.info.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddExpressDepotInfoGetResponse {
    /// 详细地址
    #[serde(default)]
    pub address: String,
    /// 仓库别名
    #[serde(default)]
    pub alias: String,
    /// 仓库地址（市编号）
    #[serde(default)]
    pub city: i32,
    /// 仓库编码
    #[serde(default)]
    pub code: String,
    /// 联系人姓名
    #[serde(default)]
    pub contact_name: String,
    /// 联系人电话
    #[serde(default)]
    pub contact_tel: String,
    /// 仓库id
    #[serde(default)]
    pub depot_id: i64,
    /// 仓库地址（区编号）
    #[serde(default)]
    pub district: i32,
    /// 仓库id（string）
    #[serde(default)]
    pub id_str: String,
    /// 仓库名称
    #[serde(default)]
    pub name: String,
    /// 其他仓库覆盖区域列表（外层key为省id；cover为该省份覆盖情况：1 半覆盖，2全覆盖；district为省中覆盖的地址：市id->区id列表）
    #[serde(default)]
    pub other_region: std::collections::BTreeMap<String, serde_json::Value>,
    /// 仓库地址（省编号）
    #[serde(default)]
    pub province: i32,
    /// 该仓库覆盖区域列表（其他仓库覆盖区域列表(外层key为省id；cover为该省份覆盖情况：1 半覆盖，2全覆盖；district为省中覆盖的地址：市id->区id列表)）
    #[serde(default)]
    pub region: std::collections::BTreeMap<String, serde_json::Value>,
    /// 仓库类型，暂时只有1
    #[serde(default)]
    pub r#type: i32,
    /// 邮编
    #[serde(default)]
    pub zip: String,
}
