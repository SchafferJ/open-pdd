//! # 电子面单取号
//!
//! 使用商家订单上的收件人信息电子面单取号
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Address {
    /// 市
    pub city: String,
    /// 国家/地区
    pub country: Option<String>,
    /// 详细地址
    pub detail: String,
    /// 区
    pub district: String,
    /// 省
    pub province: String,
    /// 街道
    pub town: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Sender {
    /// 发货地址，需要入参与 search 接口中的发货人地址信息一致
    pub address: Address,
    /// 手机号码
    pub mobile: String,
    /// 姓名
    pub name: String,
    /// 固定电话
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct TradeOrder {
    /// 代打店铺id
    pub mall_mask_id: String,
    /// 代打订单号
    pub order_mask_sn: String,
}

#[derive(Debug, Serialize)]
pub struct OrderInfo {
    /// 订单渠道平台编码
    pub order_channels_type: String,
    /// 订单列表，限制100个
    pub trade_order_list: Vec<TradeOrder>,
}

#[derive(Debug, Serialize)]
pub struct Items {
    /// 数量
    pub count: i32,
    /// 商品名称
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct PackageInfo {
    /// 快运货品描述
    pub goods_description: Option<String>,
    /// 包裹id,拆合单使用
    pub id: Option<String>,
    /// 商品信息,数量限制为100
    pub items: Vec<Items>,
    /// 快运包装方式描述
    pub packaging_description: Option<String>,
    /// 子母件总包裹数
    pub total_packages_count: Option<String>,
    /// 体积, 单位 ml
    pub volume: Option<i32>,
    /// 重量,单位 g
    pub weight: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TradeOrderInfoDtos {
    /// 物流服务内容链接
    pub logistics_services: Option<String>,
    /// 请求id
    pub object_id: String,
    /// 订单信息
    pub order_info: OrderInfo,
    /// 包裹信息
    pub package_info: PackageInfo,
    /// 标准模板模板URL
    pub template_url: String,
    /// 使用者ID
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ParamFdsWaybillGetRequest {
    /// 发货人信息
    pub sender: Sender,
    /// 取号列表
    pub trade_order_info_dtos: Vec<TradeOrderInfoDtos>,
    /// 物流公司 Code ，枚举： YTO- 圆通，ZTO-中通，YUNDA-韵达，STO-申通
    pub wp_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddFdsWaybillGetRequest {
    /// 入参信息
    pub param_fds_waybill_get_request: ParamFdsWaybillGetRequest,
}

impl RequestType for PddFdsWaybillGetRequest {
    type Response = PddFdsWaybillGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.fds.waybill.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Modules {
    /// 请求 id
    #[serde(default)]
    pub object_id: String,
    /// 快运母单号
    #[serde(default)]
    pub parent_waybill_code: String,
    /// 模板内容（模板内容加密，只需透传至打印组件
    #[serde(default)]
    pub print_data: String,
    /// 面单号
    #[serde(default)]
    pub waybill_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PddFdsWaybillGetResponse {
    /// 系统自动生成
    #[serde(default)]
    pub modules: Option<Vec<Modules>>,
}
