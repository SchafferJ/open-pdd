//! # 生成多多进宝频道推广
//!
//! 生成拼多多主站频道推广
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkResourceUrlGenRequest {
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    pub custom_parameters: Option<String>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成拼多多福利券微信小程序推广信息
    pub generate_we_app: Option<bool>,
    /// 推广位
    pub pid: String,
    /// 频道来源：4-限时秒杀,39997-充值中心, 39998-活动转链，39996-百亿补贴，39999-电器城，40000-领券中心，50005-火车票
    pub resource_type: Option<i32>,
    /// 原链接
    pub url: Option<String>,
}

impl RequestType for PddDdkResourceUrlGenRequest {
    type Response = PddDdkResourceUrlGenResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.resource.url.gen"
    }
}

#[derive(Debug, Deserialize)]
pub struct MultiUrl {
    /// 对应出参url的短链接，与url功能一致。
    #[serde(default)]
    pub short_url: String,
    /// 频道推广长链接，唤起H5页面
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SingleUrl {
    /// 对应出参url的短链接，与url功能一致。
    #[serde(default)]
    pub short_url: String,
    /// 频道推广长链接，唤起H5页面
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct WeAppInfo {
    /// 小程序id
    #[serde(default)]
    pub app_id: String,
    /// Banner图
    #[serde(default)]
    pub banner_url: String,
    /// 描述
    #[serde(default)]
    pub desc: String,
    /// 小程序path值
    #[serde(default)]
    pub page_path: String,
    /// 来源名
    #[serde(default)]
    pub source_display_name: String,
    /// 小程序标题
    #[serde(default)]
    pub title: String,
    /// 用户名
    #[serde(default)]
    pub user_name: String,
    /// 小程序icon
    #[serde(default)]
    pub we_app_icon_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkResourceUrlGenResponse {
    /// 多人团链接
    #[serde(default)]
    pub multi_url_list: Option<MultiUrl>,
    /// sign
    #[serde(default)]
    pub sign: String,
    /// 单人团链接
    #[serde(default)]
    pub single_url_list: Option<SingleUrl>,
    /// 拼多多福利券微信小程序信息
    #[serde(default)]
    pub we_app_info: Option<WeAppInfo>,
}
