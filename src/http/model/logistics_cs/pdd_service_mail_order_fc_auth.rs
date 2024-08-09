//! # 获取丰巢开平的access_token
//!
//! 获取丰巢开平的access_token
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct UrlParams {
    /// 丰巢开平app_key
    pub app_key: Option<String>,
    /// 丰巢开平app_secret
    pub app_secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PddServiceMailOrderFcAuthRequest {
    /// 拼接到url的参数
    #[serde(rename = "urlParams")]
    pub url_params: Option<UrlParams>,
    /// 请求方法
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
}

impl RequestType for PddServiceMailOrderFcAuthRequest {
    type Response = PddServiceMailOrderFcAuthResponse;

    fn get_type(&self) -> &'static str {
        "pdd.service.mail.order.fc.auth"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddServiceMailOrderFcAuthResponse;
