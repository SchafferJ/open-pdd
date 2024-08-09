//! # 商品素材创建接口
//!
//! 上传白底图长图等素材到商品
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsMaterialCreateRequest {
    /// 素材内容（一般为图片链接）
    pub content: String,
    /// 图片空间文件id
    pub file_id: i64,
    /// 商品id
    pub goods_id: i64,
    /// 素材类型（1：白底图；3：长图）
    pub material_type: i32,
}

impl RequestType for PddGoodsMaterialCreateRequest {
    type Response = PddGoodsMaterialCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.material.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsMaterialCreateResponse;
