//! # 增加仓库
//!
//! 增加仓库
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddExpressAddDepotRequest {
    /// 联系人姓名
    pub contact_name: Option<String>,
    /// 仓库详细地址5-20字
    pub depot_address: String,
    /// 别名
    pub depot_alias: String,
    /// 所在市id
    pub depot_city_id: i32,
    /// 仓库编码
    pub depot_code: String,
    /// 所在区id
    pub depot_district_id: i32,
    /// 仓库名称
    pub depot_name: String,
    /// 所在省id
    pub depot_province_id: i32,
    /// 仓库区域（ 省->市->区id列表）例如：{"34":{"396":[3383]}}Map<Integer, Map<Integer, List<Integer>>>
    /// {
    /// 1:{
    ///   2:[3,4]
    ///   }
    /// }
    pub depot_region: std::collections::BTreeMap<String, serde_json::Value>,
    /// 联系人电话
    pub telephone: Option<String>,
    /// 邮编
    pub zip_code: Option<String>,
}

impl RequestType for PddExpressAddDepotRequest {
    type Response = PddExpressAddDepotResponse;

    fn get_type(&self) -> &'static str {
        "pdd.express.add.depot"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddExpressAddDepotResponse;
