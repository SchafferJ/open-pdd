//! # 电子面单云打印更新接口
//!
//! 电子面单云打印更新接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Items {
    /// 数量
    pub count: Option<i32>,
    /// 名称
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PackageInfo {
    /// 商品
    pub items: Option<Vec<Items>>,
    /// 体积
    pub volume: Option<i32>,
    /// 重量
    pub weight: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct Address {
    /// 城市
    pub city: Option<String>,
    /// 地区/国家
    pub country: Option<String>,
    /// 详细地址
    pub detail: Option<String>,
    /// 区地址
    pub district: Option<String>,
    /// 省
    pub province: Option<String>,
    /// 街道
    pub town: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Recipient {
    /// 地址
    pub address: Option<Address>,
    /// 手机号码
    pub mobile: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 固定电话
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Sender {
    /// 手机号码
    pub mobile: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 固定电话
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ParamWaybillCloudPrintUpdateRequest {
    /// 请求表示id
    pub object_id: Option<String>,
    /// 包裹信息
    pub package_info: Option<PackageInfo>,
    /// 收件信息
    pub recipient: Option<Recipient>,
    /// 发件信息
    pub sender: Option<Sender>,
    /// 模板URL
    pub template_url: Option<String>,
    /// 面单号
    pub waybill_code: String,
    /// 物流公司CODE
    pub wp_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddWaybillUpdateRequest {
    /// param_waybill_cloud_print_update_request
    pub param_waybill_cloud_print_update_request: ParamWaybillCloudPrintUpdateRequest,
}

impl RequestType for PddWaybillUpdateRequest {
    type Response = PddWaybillUpdateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.waybill.update"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddWaybillUpdateResponse {
    /// 模板内容
    #[serde(default)]
    pub print_data: String,
    /// 面单号
    #[serde(default)]
    pub waybill_code: String,
}
