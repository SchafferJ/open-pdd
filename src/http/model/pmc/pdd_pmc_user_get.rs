//! # 获取用户已开通消息
//!
//! 获取用户已开通消息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPmcUserGetRequest {
    /// 用户唯一id
    pub owner_id: String,
}

impl RequestType for PddPmcUserGetRequest {
    type Response = PddPmcUserGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.pmc.user.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PmcUser {
    /// 用户首次开通时间
    #[serde(default)]
    pub created: String,
    /// 用户授权是否有效，0表示授权有效，1表示授权过期
    #[serde(default)]
    pub is_expire: i32,
    /// 用户最后开通时间
    #[serde(default)]
    pub modified: String,
    /// 用户ID
    #[serde(default)]
    pub owner_id: String,
    /// 用户开通的消息类型列表。如果为空表示应用开通的所有类型
    #[serde(default)]
    pub topics: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PddPmcUserGetResponse {
    /// 开通的用户数据
    #[serde(default)]
    pub pmc_user: Option<PmcUser>,
}
