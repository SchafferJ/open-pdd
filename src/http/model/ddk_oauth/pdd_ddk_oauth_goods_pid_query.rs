//! # 多多客已生成推广位信息查询
//!
//! 查询已经生成的推广位信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthGoodsPidQueryRequest {
    /// 返回的页数
    pub page: Option<i32>,
    /// 返回的每页推广位数量
    pub page_size: Option<i32>,
    /// 推广位列表，例如：["60005_612"]
    pub pid_list: Option<Vec<String>>,
}

impl RequestType for PddDdkOauthGoodsPidQueryRequest {
    type Response = PddDdkOauthGoodsPidQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.goods.pid.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PId {
    /// 推广位生成时间
    #[serde(default)]
    pub create_time: i64,
    /// 媒体id
    #[serde(default)]
    pub media_id: i64,
    /// 推广位名称
    #[serde(default)]
    pub pid_name: String,
    /// 推广位ID
    #[serde(default)]
    pub p_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthGoodsPidQueryResponse {
    /// 多多进宝推广位对象列表
    #[serde(default)]
    pub p_id_list: Option<Vec<PId>>,
    /// 返回推广位总数
    #[serde(default)]
    pub total_count: i64,
}
