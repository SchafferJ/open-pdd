//! # 草稿列表接口
//!
//! 查询店铺的商品草稿列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCommitListGetRequest {
    /// 草稿状态（0:编辑中,1:审核中,2:审核通过,3:审核驳回）
    pub check_status: i32,
    /// 商品id
    pub goods_id: Option<i64>,
    /// 页码，最多不超过100
    pub page: i32,
    /// 每页数量，最多不超过100
    pub page_size: i32,
}

impl RequestType for PddGoodsCommitListGetRequest {
    type Response = PddGoodsCommitListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.commit.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 审核时间
    #[serde(default)]
    pub checked_time: i32,
    /// 草稿状态 0:编辑中,1:审核中,2:审核通过,3:审核驳回
    #[serde(default)]
    pub check_status: i32,
    /// 草稿id
    #[serde(default)]
    pub commit_id: i64,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 商品标题
    #[serde(default)]
    pub goods_name: String,
    /// 是否新增 0:新增，1：修改
    #[serde(default)]
    pub is_shop: i32,
    /// 商家编码
    #[serde(default)]
    pub outer_goods_id: String,
    /// 驳回原因
    #[serde(default)]
    pub reject_comment: String,
    /// 提交时间
    #[serde(default)]
    pub submit_time: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCommitListGetResponse {
    /// list
    #[serde(default)]
    pub list: Option<Vec<List>>,
    /// total
    #[serde(default)]
    pub total: i64,
}
