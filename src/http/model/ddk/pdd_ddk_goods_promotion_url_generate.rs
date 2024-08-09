//! # 多多进宝推广链接生成
//!
//! 生成普通商品推广链接
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GoodsGenUrlParam {
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign: Option<String>,
    /// 需要在链接上拼接的skuIdCode列表，skuIdCode为skuId密文，由订单详情接口pdd.ddk.order.detail.get返回。要求拥有sku权限否则不生效，作用同sku_id_list，且与sku_id_list独立。此列表传入n个skuIdCode，则针对该goodsSign生成n个拼接sku_id=xxx(skuIdCode)的链接。若列表为空或者skuIdCode无效则返回普通链接
    pub sku_id_code_list: Option<Vec<String>>,
    /// 需要在链接上拼接的skuId列表，要求拥有sku权限否则不生效。拼接sku_id的链接在点击跳转商详时，自动选中对应的sku。此列表传入n个skuId，则针对该goodsSign生成n个拼接sku_id链接。若列表为空或着skuId无效（null，非正）则返回普通链接。
    pub sku_id_list: Option<Vec<i64>>,
}

#[derive(Debug, Serialize)]
pub struct PddDdkGoodsPromotionUrlGenerateRequest {
    /// 多多礼金ID
    pub cash_gift_id: Option<i64>,
    /// 自定义礼金标题，用于向用户展示渠道专属福利，不超过12个字
    pub cash_gift_name: Option<String>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。若进行cid投放，生链的时候不填充custom_parameters，后续在推广前原始链接上拼接custom_parameters。（如果使用GET请求，请使用URLEncode处理参数）
    pub custom_parameters: Option<String>,
    /// 是否生成带授权的单品链接。如果未授权，则会走授权流程
    pub generate_authority_url: Option<bool>,
    /// 是否生成店铺收藏券推广链接
    pub generate_mall_collect_coupon: Option<bool>,
    /// 是否生成qq小程序
    pub generate_qq_app: Option<bool>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成商品推广分享图，仅支持单个商品
    pub generate_share_image: Option<bool>,
    /// 获取微信ShortLink链接，仅支持单个商品，单个渠道每天生成的shortLink数量有限，请合理生成shortLink链接
    pub generate_short_link: Option<bool>,
    /// 是否生成短链接，true-是，false-否
    pub generate_short_url: Option<bool>,
    /// 是否生成拼多多福利券微信小程序推广信息
    pub generate_we_app: Option<bool>,
    /// 获取微信小程序码，仅支持单个商品
    pub generate_weixin_code: Option<bool>,
    /// 支持拼接特殊参数的商品生链参数列表。生链优先级：goods_gen_url_param_list > goods_sign_list，两者按优先级选其一。
    pub goods_gen_url_param_list: Option<Vec<GoodsGenUrlParam>>,
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

impl RequestType for PddDdkGoodsPromotionUrlGenerateRequest {
    type Response = PddDdkGoodsPromotionUrlGenerateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.goods.promotion.url.generate"
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
    /// 普通长链，微信环境下进入领券页点领券拉起小程序，浏览器环境下直接拉起APP，未安装拼多多APP时落地页点领券拉起登录页
    #[serde(default)]
    pub mobile_url: String,
    /// qq小程序信息
    #[serde(default)]
    pub qq_app_info: Option<QqAppInfo>,
    /// 使用此推广链接，用户安装拼多多APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub schema_url: String,
    /// 商品推广分享图
    #[serde(default)]
    pub share_image_url: String,
    /// 对应出参url的短链接，与url功能一致
    #[serde(default)]
    pub short_url: String,
    /// 使用此推广链接，用户安装多多团长APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub tz_schema_url: String,
    /// 普通长链。微信环境下进入领券页点领券拉起小程序，浏览器环境下优先拉起微信小程序
    #[serde(default)]
    pub url: String,
    /// 拼多多福利券微信小程序信息
    #[serde(default)]
    pub we_app_info: Option<WeAppInfo>,
    /// 微信小程序码
    #[serde(default)]
    pub weixin_code: String,
    /// 小程序短链，点击可直接唤起微信小程序
    #[serde(default)]
    pub weixin_short_link: String,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkGoodsPromotionUrlGenerateResponse {
    /// 多多进宝推广链接对象列表
    #[serde(default)]
    pub goods_promotion_url_list: Option<Vec<GoodsPromotionUrl>>,
}
