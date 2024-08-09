//! # 批量加密
//!
//! 批量加密
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Data {
    /// 明文数据
    pub data: String,
    /// 是否支持搜索
    pub search: bool,
    /// 敏感信息类型. id: 身份证号, phone: 手机号码, simple: 昵称, 地址等
    pub r#type: String,
}

#[derive(Debug, Serialize)]
pub struct PddOpenKmsEncryptBatchRequest {
    /// 要加密的数据列表, 列表大小不超过100
    pub data_list: Vec<Data>,
}

impl RequestType for PddOpenKmsEncryptBatchRequest {
    type Response = PddOpenKmsEncryptBatchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.kms.encrypt.batch"
    }
}

#[derive(Debug, Deserialize)]
pub struct DataEncrypt {
    /// 明文数据
    #[serde(default)]
    pub data: String,
    /// 加密结果
    #[serde(default)]
    pub data_encrypt: String,
    /// 是否支持搜索
    #[serde(default)]
    pub search: bool,
    /// 是否成功
    #[serde(default)]
    pub success: bool,
    /// 敏感信息类型. id: 身份证号, phone: 手机号码, simple: 昵称, 地址等
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct PddOpenKmsEncryptBatchResponse {
    /// list
    #[serde(default)]
    pub data_encrypt_list: Option<Vec<DataEncrypt>>,
}
