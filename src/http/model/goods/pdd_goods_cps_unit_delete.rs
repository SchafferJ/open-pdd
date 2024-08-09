//! # 删除单品计划接口
//!
//! 删除单品计划功能
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCpsUnitDeleteRequest {
    /// 商品id
    pub goods_id: i64,
}

impl RequestType for PddGoodsCpsUnitDeleteRequest {
    type Response = PddGoodsCpsUnitDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cps.unit.delete"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCpsUnitDeleteResponse {
    /// 是否成功
    #[serde(default)]
    pub success: bool,
}
