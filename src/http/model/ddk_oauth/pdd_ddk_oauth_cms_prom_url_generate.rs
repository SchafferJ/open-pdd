//! # 生成商城推广链接接口
//!
//! 生成商城推广链接接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthCmsPromUrlGenerateRequest {
    /// 0, "1.9包邮"；1, "今日爆款"； 2, "品牌清仓"； 4,"PC端专属商城(已下线,会生成默认商城)"
    pub channel_type: Option<i32>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    pub custom_parameters: Option<String>,
    /// 是否生成手机跳转链接。true-是，false-否，默认false
    pub generate_mobile: Option<bool>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成短链接，true-是，false-否
    pub generate_short_url: Option<bool>,
    /// 是否生成拼多多福利券微信小程序推广信息
    pub generate_we_app: Option<bool>,
    /// 搜索关键词
    pub keyword: Option<String>,
    /// 单人团多人团标志。true-多人团，false-单人团 默认false
    pub multi_group: Option<bool>,
    /// 推广位列表，例如：["60005_612"]
    pub p_id_list: Vec<String>,
}

impl RequestType for PddDdkOauthCmsPromUrlGenerateRequest {
    type Response = PddDdkOauthCmsPromUrlGenerateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.cms.prom.url.generate"
    }
}

#[derive(Debug, Deserialize)]
pub struct MultiUrl {
    /// 双人团唤醒拼多多app短链接
    #[serde(default)]
    pub mobile_short_url: String,
    /// 双人团唤醒拼多多app长链接
    #[serde(default)]
    pub mobile_url: String,
    /// schema链接，用户安装拼多多APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub schema_url: String,
    /// 双人团短链接
    #[serde(default)]
    pub short_url: String,
    /// 使用此推广链接，用户安装多多团长APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub tz_schema_url: String,
    /// 双人团长链接，唤起H5页面
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SingleUrl {
    /// 唤醒拼多多app短链接
    #[serde(default)]
    pub mobile_short_url: String,
    /// 唤醒拼多多app长链接
    #[serde(default)]
    pub mobile_url: String,
    /// schema链接，用户安装拼多多APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub schema_url: String,
    /// 对应出参url的短链接，与url功能一致。
    #[serde(default)]
    pub short_url: String,
    /// 使用此推广链接，用户安装多多团长APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub tz_schema_url: String,
    /// 普通推广长链接，唤起H5页面
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
    /// 小程序图片
    #[serde(default)]
    pub we_app_icon_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    /// 唤醒拼多多app短链
    #[serde(default)]
    pub mobile_short_url: String,
    /// 唤醒拼多多app链接
    #[serde(default)]
    pub mobile_url: String,
    /// 多人团唤醒拼多多app链接
    #[serde(default)]
    pub multi_group_mobile_short_url: String,
    /// 多人团唤醒拼多多app长链接
    #[serde(default)]
    pub multi_group_mobile_url: String,
    /// 多人团短链，唤起H5页面
    #[serde(default)]
    pub multi_group_short_url: String,
    /// 多人团长链，唤起H5页面
    #[serde(default)]
    pub multi_group_url: String,
    /// 双人团链接列表
    #[serde(default)]
    pub multi_url_list: Option<MultiUrl>,
    /// h5短链接
    #[serde(default)]
    pub short_url: String,
    /// CPSsign
    #[serde(default)]
    pub sign: String,
    /// 单人团链接列表
    #[serde(default)]
    pub single_url_list: Option<SingleUrl>,
    /// 普通推广长链接，唤起H5页面
    #[serde(default)]
    pub url: String,
    /// 拼多多福利券微信小程序信息
    #[serde(default)]
    pub we_app_info: Option<WeAppInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthCmsPromUrlGenerateResponse {
    /// total
    #[serde(default)]
    pub total: i32,
    /// 链接列表
    #[serde(default)]
    pub url_list: Option<Vec<Url>>,
}
