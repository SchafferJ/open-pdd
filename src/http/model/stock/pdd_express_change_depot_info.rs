//! # 修改仓库信息
//!
//! 修改仓库信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddExpressChangeDepotInfoRequest {
    /// 仓库id
    pub depot_id: i64,
    /// 仓库编码
    pub depot_code: Option<String>,
    /// 仓库名称
    pub depot_name: Option<String>,
    /// 别名
    pub depot_alias: Option<String>,
    /// 所在省id
    pub depot_province_id: Option<i32>,
    /// 所在市id
    pub depot_city_id: Option<i32>,
    /// 所在区id
    pub depot_district_id: Option<i32>,
    /// 仓库详细地址 5-20字
    pub depot_address: Option<String>,
    /// 联系人姓名
    pub contact_name: Option<String>,
    /// 联系人电话
    pub telephone: Option<String>,
    /// 仓库区域（ 省->市->区id列表）例如：{"34":{"396":[3383]}}Map<Integer, Map<Integer, List<Integer>>>
    pub depot_region: Option<String>,
}

impl RequestType for PddExpressChangeDepotInfoRequest {
    type Response = PddExpressChangeDepotInfoResponse;

    fn get_type(&self) -> &'static str {
        "pdd.express.change.depot.info"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddExpressChangeDepotInfoResponse;
