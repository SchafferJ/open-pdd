//! # 刷新Access Token
//!
//! 根据refresh_token重新生成token
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPopAuthTokenRefreshRequest {
    /// grantType==refresh_token 时需要
    pub refresh_token: String,
}

impl RequestType for PddPopAuthTokenRefreshRequest {
    type Response = PddPopAuthTokenRefreshResponse;

    fn get_type(&self) -> &'static str {
        "pdd.pop.auth.token.refresh"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPopAuthTokenRefreshResponse {
    /// access_token
    #[serde(default)]
    pub access_token: String,
    /// access_token过期时间点
    #[serde(default)]
    pub expires_at: i64,
    /// access_token过期时间段，10（表示10秒后过期）
    #[serde(default)]
    pub expires_in: i32,
    /// 商家店铺id
    #[serde(default)]
    pub owner_id: String,
    /// 商家账号名称
    #[serde(default)]
    pub owner_name: String,
    /// r1级别API或字段的访问过期时间点
    #[serde(default)]
    pub r1_expires_at: i64,
    /// r1级别API或字段的访问过期时间；	 10（表示10秒后过期）
    #[serde(default)]
    pub r1_expires_in: i32,
    /// r2级别API或字段的访问过期时间点
    #[serde(default)]
    pub r2_expires_at: i64,
    /// r2级别API或字段的访问过期时间；10（表示10秒后过期）
    #[serde(default)]
    pub r2_expires_in: i32,
    /// refresh token，可用来刷新access_token
    #[serde(default)]
    pub refresh_token: String,
    /// Refresh token过期时间点
    #[serde(default)]
    pub refresh_token_expires_at: i64,
    /// refresh_token过期时间段，10表示10秒后过期
    #[serde(default)]
    pub refresh_token_expires_in: i32,
    /// 接口列表
    #[serde(default)]
    pub scope: Vec<String>,
    /// w1级别API或字段的访问过期时间点
    #[serde(default)]
    pub w1_expires_at: i64,
    /// w1级别API或字段的访问过期时间；	 10（表示10秒后过期）
    #[serde(default)]
    pub w1_expires_in: i32,
    /// w2级别API或字段的访问过期时间点
    #[serde(default)]
    pub w2_expires_at: i64,
    /// w2级别API或字段的访问过期时间；10（表示10秒后过期）
    #[serde(default)]
    pub w2_expires_in: i32,
}
