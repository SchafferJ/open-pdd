//! # 仓库列表
//!
//! 仓库列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddExpressDepotListGetRequest {
    /// 分页数据size
    pub length: i64,
    /// 分页数据起始位置
    pub start: i64,
}

impl RequestType for PddExpressDepotListGetRequest {
    type Response = PddExpressDepotListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.express.depot.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Depot {
    /// 仓库详细地址
    #[serde(default)]
    pub address: String,
    /// 仓库别名
    #[serde(default)]
    pub alias: String,
    /// 仓库所在市
    #[serde(default)]
    pub city: i32,
    /// 仓库编码
    #[serde(default)]
    pub code: String,
    /// 联系人名字
    #[serde(default)]
    pub contact_name: String,
    /// 联系人电话
    #[serde(default)]
    pub contact_tel: String,
    /// 仓库Id
    #[serde(default)]
    pub depot_id: i64,
    /// 仓库所在区
    #[serde(default)]
    pub district: i32,
    /// 仓库Id2
    #[serde(default)]
    pub id_str: String,
    /// 仓库名称
    #[serde(default)]
    pub name: String,
    /// 仓库所在省
    #[serde(default)]
    pub province: i32,
    /// 覆盖信息,key是覆盖的省id;value中1表示局部，2表示全部覆盖
    #[serde(default)]
    pub province_map: std::collections::BTreeMap<String, serde_json::Value>,
    /// 仓库类型
    #[serde(default)]
    pub r#type: i32,
    /// 邮编
    #[serde(default)]
    pub zip: String,
}

#[derive(Debug, Deserialize)]
pub struct PddExpressDepotListGetResponse {
    /// 仓库数量
    #[serde(default)]
    pub count: i32,
    /// 仓库列表
    #[serde(default)]
    pub depot_list: Option<Vec<Depot>>,
}
