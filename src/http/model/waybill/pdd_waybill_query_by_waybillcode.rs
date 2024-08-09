//! # 通过面单号查询面单信息
//!
//! 通过面单号查询面单信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Param {
    /// 请求id
    pub object_id: String,
    /// 电子面单号
    pub waybill_code: String,
    /// 快递公司code
    pub wp_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddWaybillQueryByWaybillcodeRequest {
    /// 系统自动生成
    pub param_list: Vec<Param>,
}

impl RequestType for PddWaybillQueryByWaybillcodeRequest {
    type Response = PddWaybillQueryByWaybillcodeResponse;

    fn get_type(&self) -> &'static str {
        "pdd.waybill.query.by.waybillcode"
    }
}

#[derive(Debug, Deserialize)]
pub struct WaybillCloudPrintResponse {
    /// 请求id
    #[serde(default)]
    pub object_id: String,
    /// 面单信息
    #[serde(default)]
    pub print_data: String,
    /// 面单号
    #[serde(default)]
    pub waybill_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Modules {
    /// 面单查询结构体
    #[serde(default)]
    pub waybill_cloud_print_response: Option<WaybillCloudPrintResponse>,
}

#[derive(Debug, Deserialize)]
pub struct PddWaybillQueryByWaybillcodeResponse {
    /// 查询返回值
    #[serde(default)]
    pub modules: Option<Vec<Modules>>,
}
