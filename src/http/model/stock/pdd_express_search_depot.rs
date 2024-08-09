//! # 根据仓库名称和仓库编码查询仓库
//!
//! 根据仓库名称和仓库编码查询仓库
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddExpressSearchDepotRequest {
    /// 仓库编码
    pub code: String,
    /// 10 分页数据size
    pub length: i32,
    /// 仓库名称
    pub name: String,
    /// 0 分页数据起始位置
    pub start: i32,
}

impl RequestType for PddExpressSearchDepotRequest {
    type Response = PddExpressSearchDepotResponse;

    fn get_type(&self) -> &'static str {
        "pdd.express.search.depot"
    }
}

#[derive(Debug, Deserialize)]
pub struct Depot {
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
    /// 仓库地址（省编号）
    #[serde(default)]
    pub province: i32,
    /// 覆盖信息，省：全覆盖
    #[serde(default)]
    pub province_map: std::collections::BTreeMap<String, serde_json::Value>,
    /// 仓库类型，暂时只有1
    #[serde(default)]
    pub r#type: i32,
    /// 邮编
    #[serde(default)]
    pub zip: String,
}

#[derive(Debug, Deserialize)]
pub struct PddExpressSearchDepotResponse {
    /// 数量
    #[serde(default)]
    pub count: i64,
    /// 仓库列表
    #[serde(default)]
    pub depot_list: Option<Vec<Depot>>,
}
