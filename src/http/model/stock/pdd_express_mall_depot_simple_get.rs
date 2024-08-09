//! # 商家全部仓库的简要信息列表
//!
//! 商家全部仓库的简要信息列表(无业务入参)
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddExpressMallDepotSimpleGetRequest;

impl RequestType for PddExpressMallDepotSimpleGetRequest {
    type Response = PddExpressMallDepotSimpleGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.express.mall.depot.simple.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddExpressMallDepotSimpleGetResponse {
    /// 仓库Id
    #[serde(default)]
    pub depot_id: i64,
    /// 仓库id的string表示
    #[serde(default)]
    pub depot_str: String,
    /// 仓库类型(暂时1表示自建仓)
    #[serde(default)]
    pub depot_type: i32,
    /// 仓库名称
    #[serde(default)]
    pub name: String,
}
