//! # 文件详情查询
//!
//! 文件详情查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsFileInfoGetRequest {
    /// url列表
    pub url_list: Option<Vec<String>>,
}

impl RequestType for PddGoodsFileInfoGetRequest {
    type Response = PddGoodsFileInfoGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.file.info.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 文件id
    #[serde(default)]
    pub file_id: i64,
    /// 文件状态(-2:上传失败,-1:转码失败,0:转码中,1:审核中,2:审核通过,3:审核驳回)
    #[serde(default)]
    pub status: i32,
    /// 文件链接
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsFileInfoGetResponse {
    #[serde(default)]
    pub list: Option<Vec<List>>,
}
