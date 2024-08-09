//! # 生成多多进宝推广链接
//!
//! 生成普通商品推广链接
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthGoodsPromUrlGenerateRequest {
    /// 多多礼金ID
    pub cash_gift_id: Option<i64>,
    /// 自定义礼金标题，用于向用户展示渠道专属福利，不超过12个字
    pub cash_gift_name: Option<String>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    pub custom_parameters: Option<String>,
    /// 是否使用多多客专属推广计划
    pub force_duo_id: Option<bool>,
    /// 是否生成带授权的单品链接。如果未授权，则会走授权流程
    pub generate_authority_url: Option<bool>,
    /// 是否生成店铺收藏券推广链接
    pub generate_mall_collect_coupon: Option<bool>,
    /// 是否生成qq小程序
    pub generate_qq_app: Option<bool>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成短链接，true-是，false-否
    pub generate_short_url: Option<bool>,
    /// 是否生成拼多多福利券微信小程序推广信息
    pub generate_we_app: Option<bool>,
    /// 商品goodsSign列表，例如：["c9r2omogKFFAc7WBwvbZU1ikIb16_J3CTa8HNN"]，支持批量生链。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign_list: Option<Vec<String>>,
    /// 素材ID，可以通过商品详情接口获取商品素材信息
    pub material_id: Option<String>,
    /// true--生成多人团推广链接 false--生成单人团推广链接（默认false）1、单人团推广链接：用户访问单人团推广链接，可直接购买商品无需拼团。2、多人团推广链接：用户访问双人团推广链接开团，若用户分享给他人参团，则开团者和参团者的佣金均结算给推手
    pub multi_group: Option<bool>,
    /// 推广位ID
    pub p_id: String,
    /// 搜索id，建议填写，提高收益。来自pdd.ddk.goods.recommend.get、pdd.ddk.goods.search、pdd.ddk.top.goods.list.query等接口
    pub search_id: Option<String>,
    /// 特殊参数
    pub special_params: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// 招商多多客ID
    pub zs_duo_id: Option<i64>,
    /// 生成商品链接类型 0-默认 1-百补相似品列表
    pub url_type: Option<i32>,
}

impl RequestType for PddDdkOauthGoodsPromUrlGenerateRequest {
    type Response = PddDdkOauthGoodsPromUrlGenerateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.goods.prom.url.generate"
    }
}

#[derive(Debug, Deserialize)]
pub struct QqAppInfo {
    /// 拼多多小程序id
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
    /// 小程序icon
    #[serde(default)]
    pub qq_app_icon_url: String,
    /// 来源名
    #[serde(default)]
    pub source_display_name: String,
    /// 小程序标题
    #[serde(default)]
    pub title: String,
    /// 用户名
    #[serde(default)]
    pub user_name: String,
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
pub struct GoodsPromotionUrl {
    /// 对应出参mobile_url的短链接，与mobile_url功能一致。
    #[serde(default)]
    pub mobile_short_url: String,
    /// 使用此推广链接，用户安装微信的情况下，默认拉起拼多多福利券微信小程序，否则唤起H5页面
    #[serde(default)]
    pub mobile_url: String,
    /// qq小程序信息
    #[serde(default)]
    pub qq_app_info: Option<QqAppInfo>,
    /// 使用此推广链接，用户安装拼多多APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub schema_url: String,
    /// 对应出参url的短链接，与url功能一致
    #[serde(default)]
    pub short_url: String,
    /// 使用此推广链接，用户安装多多团长APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub tz_schema_url: String,
    /// 普通推广长链接，唤起H5页面
    #[serde(default)]
    pub url: String,
    /// 拼多多福利券微信小程序信息
    #[serde(default)]
    pub we_app_info: Option<WeAppInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthGoodsPromUrlGenerateResponse {
    /// 多多进宝推广链接对象列表
    #[serde(default)]
    pub goods_promotion_url_list: Option<Vec<GoodsPromotionUrl>>,
}
