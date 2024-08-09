//! # 商品素材删除接口
//!
//! 删除素材
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsMaterialDeleteRequest {
    /// 素材id
    pub material_id: i64,
}

impl RequestType for PddGoodsMaterialDeleteRequest {
    type Response = PddGoodsMaterialDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.material.delete"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsMaterialDeleteResponse;
