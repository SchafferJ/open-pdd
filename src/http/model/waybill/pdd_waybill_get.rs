//! # 电子面单云打印接口
//!
//! 电子面单云打印接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Address {
    /// 城市，仅支持非空值
    pub city: String,
    /// 国家/地区
    pub country: Option<String>,
    /// 详细地址，仅支持非空值
    pub detail: String,
    /// 区，仅支持非空值
    pub district: String,
    /// 省，仅支持非空值
    pub province: String,
    /// 街道
    pub town: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Sender {
    /// 地址
    pub address: Address,
    /// 手机号码
    pub mobile: Option<String>,
    /// 姓名
    pub name: String,
    /// 固定电话
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderInfo {
    /// 订单渠道平台编码 拼多多-PDD，淘宝-TB，天猫-TM，京东-JD，阿里巴巴-ALBB，有赞-YZ，微店-WD，蘑菇街-MGJ，云集-YJ，贝贝网-BB，转转-ZZ，快手小店-KS，当当网-DD，小米有品-XMYP，寺库-SK，聚美优品-JM，蜜芽-MY，小红书-XHS，萌推-MT，唯品会-WPH，拍拍-PP，ebay-EBAY，亚马逊-AMAZON，苏宁-SN，国美-GM，1号店-YHD，凡客-VANCL，邮乐-YL，优购-YG，乐蜂-LF，聚尚-JS，拍鞋-PX，银泰-YT，抖音-DY，其他-OTHERS
    pub order_channels_type: String,
    /// 订单号,数量限制100
    pub trade_order_list: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Items {
    /// 数量
    pub count: i32,
    /// 名称
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
    pub total_packages_count: Option<i32>,
    /// 体积, 单位 ml
    pub volume: Option<i64>,
    /// 重量,单位 g
    pub weight: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct AddressRequest {
    /// 城市，仅支持非空值
    pub city: String,
    /// 国家/地区
    pub country: Option<String>,
    /// 详细地址，仅支持非空值
    pub detail: String,
    /// 区，仅支持非空值
    pub district: String,
    /// 省，仅支持非空值
    pub province: String,
    /// 街道
    pub town: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Recipient {
    /// 地址
    pub address: AddressRequest,
    /// 手机号码
    pub mobile: Option<String>,
    /// 姓名
    pub name: String,
    /// 固定电话
    pub phone: Option<String>,
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
    /// 收件人信息
    pub recipient: Option<Recipient>,
    /// 标准模板模板URL
    pub template_url: String,
    /// 使用者ID
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ParamWaybillCloudPrintApplyNewRequest {
    /// 设定取号返回的云打印报文是否加密
    pub need_encrypt: Option<bool>,
    /// 发货人信息
    pub sender: Sender,
    /// 请求面单信息，数量限制为10
    pub trade_order_info_dtos: Option<Vec<TradeOrderInfoDtos>>,
    /// 物流公司Code
    pub wp_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddWaybillGetRequest {
    /// 入参信息
    pub param_waybill_cloud_print_apply_new_request: ParamWaybillCloudPrintApplyNewRequest,
}

impl RequestType for PddWaybillGetRequest {
    type Response = PddWaybillGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.waybill.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Modules {
    /// 请求id
    #[serde(default)]
    pub object_id: String,
    /// 快运母单号
    #[serde(default)]
    pub parent_waybill_code: String,
    /// 面单信息
    #[serde(default)]
    pub print_data: String,
    /// 面单号
    #[serde(default)]
    pub waybill_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PddWaybillGetResponse {
    /// 系统自动生成
    #[serde(default)]
    pub modules: Option<Vec<Modules>>,
}
