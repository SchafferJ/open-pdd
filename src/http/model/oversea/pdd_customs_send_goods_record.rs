//! # 海淘服务商上传商品备案信息
//!
//! 海淘服务商上传商品备案信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Goods {
    /// 条形码
    pub bar_code: Option<String>,
    /// 保税仓名称
    pub bonded_warehouse_name: Option<String>,
    /// 品牌中文名称
    pub brand_chinese_name: Option<String>,
    /// 品牌英文名称
    pub brand_english_name: Option<String>,
    /// 品类
    pub category: Option<String>,
    /// 消费税率，单位%
    pub consumption_tax_rate: Option<f64>,
    /// 成本价（RMB）
    pub cost_price: Option<f64>,
    /// 海关关区代码
    pub customs_code: Option<String>,
    /// 备案电商企业的海关注册登记名称(备案的电商企业名称)
    pub ebc_name: Option<String>,
    /// 账册编号
    pub ems_no: Option<String>,
    /// 保质期
    pub expiration_date: Option<String>,
    /// 毛重（KG）
    pub gross_weight: Option<f64>,
    /// 海关HS code
    pub hs_code: Option<String>,
    /// 备案商品图片链接
    pub img_url: Option<String>,
    /// 电商企业的商品编号(skuId非pdd skuId)
    pub item_no: Option<String>,
    /// 物料号
    pub item_record_no: Option<String>,
    /// 生产企业名称
    pub manufacturing_company_name: Option<String>,
    /// 生产企业注册号
    pub manufacturing_company_registration_no: Option<String>,
    /// 生产厂家地址（奶制品必填）
    pub manufacturing_factory_address: Option<String>,
    /// 净重（KG）
    pub net_weight: Option<f64>,
    /// 海关口岸代码
    pub port_code: Option<String>,
    /// 生产国代码
    pub producing_country: Option<String>,
    /// 产品国检备案编号
    pub product_record_no: Option<String>,
    /// 法定第一数量
    pub qty1: Option<f64>,
    /// 法定第二数量
    pub qty2: Option<f64>,
    /// 备案商品中文名称
    pub record_chinese_name: Option<String>,
    /// 备案商品英文名称
    pub record_english_name: Option<String>,
    /// 商品规格型号(报文gmodel)
    pub record_model: Option<String>,
    /// 型号
    pub specification: Option<String>,
    /// 库存数量
    pub stock: Option<i64>,
    /// 库存时间
    pub stock_time: Option<String>,
    /// 关税税率,单位%
    pub tariff_rate: Option<f64>,
    /// 申报单位代码
    pub unit: Option<String>,
    /// 法定第一单位代码
    pub unit1: Option<String>,
    /// 法定第二单位代码
    pub unit2: Option<String>,
    /// 单价（RMB）
    pub unit_price: Option<f64>,
    /// 增值税率，单位%
    pub value_added_tax_rate: Option<f64>,
    /// 供应商名称
    pub vendor_name: Option<String>,
    /// 备案仓储企业代码
    pub wc_code: Option<String>,
    /// 备案仓储企业的海关注册登记名称
    pub wc_name: Option<String>,
    /// 网络链接
    pub website: Option<String>,
    /// 包装方式
    pub wrap_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Request {
    /// 备案商品列表
    pub goods_list: Option<Vec<Goods>>,
}

#[derive(Debug, Serialize)]
pub struct PddCustomsSendGoodsRecordRequest {
    /// 上传备案商品请求
    pub request: Request,
}

impl RequestType for PddCustomsSendGoodsRecordRequest {
    type Response = PddCustomsSendGoodsRecordResponse;

    fn get_type(&self) -> &'static str {
        "pdd.customs.send.goods.record"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddCustomsSendGoodsRecordResponse {
    #[serde(default)]
    pub error_code: i32,
    #[serde(default)]
    pub error_msg: String,
    #[serde(default)]
    pub result: String,
    #[serde(default)]
    pub success: bool,
}
