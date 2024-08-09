//! # 商品素材列表查询
//!
//! 查询商品素材列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsMaterialQueryRequest {
    /// 商品id列表
    pub goods_id_list: Vec<i64>,
    /// 素材类型列表
    pub type_list: Vec<i64>,
}

impl RequestType for PddGoodsMaterialQueryRequest {
    type Response = PddGoodsMaterialQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.material.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsMaterialQueryResponse {
    /// 申诉审核信息
    #[serde(default)]
    pub appeal_check_comment: String,
    /// 审核信息
    #[serde(default)]
    pub check_comment: String,
    /// 审核状态（1：待审核；2：通过；3：驳回；101：申诉待审核；102：申诉通过；103：申诉驳回）
    #[serde(default)]
    pub check_status: i32,
    /// 素材内容
    #[serde(default)]
    pub content: String,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 素材id
    #[serde(default)]
    pub material_id: i64,
    /// 线上素材
    #[serde(default)]
    pub online_material: String,
    /// 素材类型（1：白底图；3：长图）
    #[serde(default)]
    pub r#type: i32,
}
