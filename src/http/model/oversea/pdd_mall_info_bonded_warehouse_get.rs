//! # 保税仓信息查询接口
//!
//! 查询商家的所有保税仓信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoBondedWarehouseGetRequest;

impl RequestType for PddMallInfoBondedWarehouseGetRequest {
    type Response = PddMallInfoBondedWarehouseGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.bonded.warehouse.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    /// 保税仓标识
    #[serde(default)]
    pub bonded_warehouse_key: String,
    /// 保税仓名字
    #[serde(default)]
    pub bonded_warehouse_name: String,
    /// 清关服务商
    #[serde(default)]
    pub customs_clearance_service_provider_list: Vec<String>,
    /// 报关海关
    #[serde(default)]
    pub customs_declaration_location: String,
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoBondedWarehouseGetResponse {
    /// list
    #[serde(default)]
    pub info_list: Option<Vec<Info>>,
}
