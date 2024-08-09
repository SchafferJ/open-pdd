//! # 多多进宝转链接口
//!
//! 本功能适用于采集群等场景。将其他推广者的推广链接转换成自己的；通过此api，可以将他人的招商推广链接，转换成自己的招商推广链接。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkGoodsZsUnitUrlGenRequest {
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。若进行cid投放，转链的时候不填充custom_parameters，后续在推广前原始链接上拼接custom_parameters。（如果使用GET请求，请使用URLEncode处理参数）
    pub custom_parameters: Option<String>,
    /// 是否生成微信shortlink链接，仅支持单品，单个渠道每天生成的shortLink数量有限，请合理生成shortLink链接
    pub generate_short_link: Option<bool>,
    /// 渠道id
    pub pid: String,
    /// 需转链的链接，支持拼多多商品链接、进宝长链/短链（即为pdd.ddk.goods.promotion.url.generate接口生成的长短链）
    pub source_url: String,
}

impl RequestType for PddDdkGoodsZsUnitUrlGenRequest {
    type Response = PddDdkGoodsZsUnitUrlGenResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.goods.zs.unit.url.gen"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkGoodsZsUnitUrlGenResponse {
    /// 对应出参mobile_url的短链接，与mobile_url功能一致
    #[serde(default)]
    pub mobile_short_url: String,
    /// 普通长链，微信环境下进入领券页点领券拉起小程序，浏览器环境下直接拉起APP，未安装拼多多APP时落地页点领券拉起登录页
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
    /// 对应出参url的短链接，与url功能一致
    #[serde(default)]
    pub short_url: String,
    /// 普通长链。微信环境下进入领券页点领券拉起小程序，浏览器环境下优先拉起微信小程序
    #[serde(default)]
    pub url: String,
    /// 小程序短链，点击可直接唤起微信小程序
    #[serde(default)]
    pub weixin_short_link: String,
}
