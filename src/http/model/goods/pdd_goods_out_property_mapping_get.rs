//! # 站内外属性映射接口
//!
//! 开平侧商家在搬家，根据入参类目+站外属性/站外属性值返回启用的站内外属性映射中的站内属性/站内属性值
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsOutPropertyMappingGetRequest {
    /// 拼多多叶子类目id
    pub cat_id: i64,
    /// 外部平台属性名称
    pub out_property_name: String,
    /// 外部平台属性值名称
    pub out_property_value_name: Option<String>,
}

impl RequestType for PddGoodsOutPropertyMappingGetRequest {
    type Response = PddGoodsOutPropertyMappingGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.out.property.mapping.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(default)]
    pub out_property_name: String,
    #[serde(default)]
    pub out_property_value_name: String,
    #[serde(default)]
    pub property_value_id: i64,
    #[serde(default)]
    pub property_value_name: String,
    #[serde(default)]
    pub ref_property_id: i64,
    #[serde(default)]
    pub ref_property_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsOutPropertyMappingGetResponse {
    #[serde(default)]
    pub property: Option<Vec<Property>>,
}
