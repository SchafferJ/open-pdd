//! # 查询面单服务订购及面单使用情况
//!
//! 查询面单服务订购及面单使用情况
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddWaybillSearchRequest {
    /// 物流公司code
    pub wp_code: Option<String>,
}

impl RequestType for PddWaybillSearchRequest {
    type Response = PddWaybillSearchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.waybill.search"
    }
}

#[derive(Debug, Deserialize)]
pub struct ServiceAttributes {
    /// 属性code
    #[serde(default)]
    pub attribute_code: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: String,
    /// 属性类型
    #[serde(default)]
    pub attribute_type: String,
    /// 属性描述
    #[serde(default)]
    pub type_desc: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceInfoCols {
    /// 是否必须
    #[serde(default)]
    pub required: bool,
    /// 服务属性类型列表
    #[serde(default)]
    pub service_attributes: Option<Vec<ServiceAttributes>>,
    /// 服务code
    #[serde(default)]
    pub service_code: String,
    /// 服务描述
    #[serde(default)]
    pub service_desc: String,
    /// 服务名称
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ShippAddressCols {
    /// 市名称（二级地址）
    #[serde(default)]
    pub city: String,
    /// 详细地址
    #[serde(default)]
    pub detail: String,
    /// 区名称（三级地址）
    #[serde(default)]
    pub district: String,
    /// 省名称（一级地址）
    #[serde(default)]
    pub province: String,
    /// 国家/地区
    #[serde(default)]
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct VasAccountCols {
    /// 账户类型描述
    #[serde(default)]
    pub account_type_desc: String,
    /// 电子面单余额数量
    #[serde(default)]
    pub quantity: i64,
    /// 已用面单数量
    #[serde(default)]
    pub allocated_quantity: i64,
    /// 取消的面单总数
    #[serde(default)]
    pub cancel_quantity: i64,
    /// 已回收用面单数量
    #[serde(default)]
    pub recycled_quantity: i64,
}

#[derive(Debug, Deserialize)]
pub struct BranchAccountCols {
    /// 已用面单数量
    #[serde(default)]
    pub allocated_quantity: i64,
    /// 网点Code
    #[serde(default)]
    pub branch_code: String,
    /// 网点名称
    #[serde(default)]
    pub branch_name: String,
    /// 取消的面单总数
    #[serde(default)]
    pub cancel_quantity: i64,
    /// 电子面单余额数量
    #[serde(default)]
    pub quantity: i64,
    /// 已回收用面单数量
    #[serde(default)]
    pub recycled_quantity: i64,
    /// 服务类型列表
    #[serde(default)]
    pub service_info_cols: Option<Vec<ServiceInfoCols>>,
    /// 当前网点下的发货地址
    #[serde(default)]
    pub shipp_address_cols: Option<Vec<ShippAddressCols>>,
    /// 增值服务账号
    #[serde(default)]
    pub vas_account_cols: Option<Vec<VasAccountCols>>,
}

#[derive(Debug, Deserialize)]
pub struct WaybillApplySubscriptionCols {
    /// wp网点信息及对应的商家的发货信息
    #[serde(default)]
    pub branch_account_cols: Option<Vec<BranchAccountCols>>,
    /// 快递公司ID
    #[serde(default)]
    pub wp_code: String,
    /// 物流服务商业务类型
    #[serde(default)]
    pub wp_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddWaybillSearchResponse {
    /// WP网点信息及对应的商家的发货信息
    #[serde(default)]
    pub waybill_apply_subscription_cols: Option<Vec<WaybillApplySubscriptionCols>>,
}
