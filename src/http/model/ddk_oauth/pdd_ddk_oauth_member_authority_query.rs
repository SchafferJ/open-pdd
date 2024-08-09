//! # 查询是否绑定备案
//!
//! 通过pid和自定义参数来查询是否已经绑定备案
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthMemberAuthorityQueryRequest {
    /// 推广位id
    pub pid: Option<String>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    pub custom_parameters: Option<String>,
}

impl RequestType for PddDdkOauthMemberAuthorityQueryRequest {
    type Response = PddDdkOauthMemberAuthorityQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.member.authority.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthMemberAuthorityQueryResponse {
    /// 1-已绑定；0-未绑定
    #[serde(default)]
    pub bind: i32,
}
