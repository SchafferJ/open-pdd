//! # 多多进宝转链接口
//!
//! 本功能适用于采集群等场景。将其他推广者的推广链接转换成自己的；通过此api，可以将他人的招商推广链接，转换成自己的招商推广链接。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthGoodsZsUnitUrlGenRequest {
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    pub custom_parameters: Option<String>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 渠道id
    pub pid: String,
    /// 需转链的链接，支持拼多多商品链接、进宝长链/短链（即为pdd.ddk.goods.promotion.url.generate接口生成的长短链）
    pub source_url: String,
}

impl RequestType for PddDdkOauthGoodsZsUnitUrlGenRequest {
    type Response = PddDdkOauthGoodsZsUnitUrlGenResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.goods.zs.unit.url.gen"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthGoodsZsUnitUrlGenResponse {
    /// 推广短链接（可唤起拼多多app）
    #[serde(default)]
    pub mobile_short_url: String,
    /// 推广长链接（唤起拼多多app）
    #[serde(default)]
    pub mobile_url: String,
    /// 推广短链接（唤起拼多多app）
    #[serde(default)]
    pub multi_group_mobile_short_url: String,
    /// 推广长链接（可唤起拼多多app）
    #[serde(default)]
    pub multi_group_mobile_url: String,
    /// 双人团推广短链接
    #[serde(default)]
    pub multi_group_short_url: String,
    /// 双人团推广长链接
    #[serde(default)]
    pub multi_group_url: String,
    /// schema的链接
    #[serde(default)]
    pub schema_url: String,
    /// 单人团推广短链接
    #[serde(default)]
    pub short_url: String,
    /// 单人团推广长链接
    #[serde(default)]
    pub url: String,
}
