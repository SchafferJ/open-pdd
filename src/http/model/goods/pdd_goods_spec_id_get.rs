//! # 生成商家自定义的规格
//!
//! 生成商家自定义的规格
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsSpecIdGetRequest {
    /// 拼多多标准规格ID，可以通过pdd.goods.spec.get接口获取
    pub parent_spec_id: i64,
    /// 商家编辑的规格值，如颜色规格下设置白色属性
    pub spec_name: String,
}

impl RequestType for PddGoodsSpecIdGetRequest {
    type Response = PddGoodsSpecIdGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.spec.id.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsSpecIdGetResponse {
    /// 自定义规格所属的规格ID
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 生成的自定义规格名称
    #[serde(default)]
    pub spec_name: String,
    /// 生成的自定义规格ID
    #[serde(default)]
    pub spec_id: i64,
}
