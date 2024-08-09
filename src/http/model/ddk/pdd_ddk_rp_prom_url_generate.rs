//! # 生成营销工具推广链接
//!
//! 生成营销工具推广链接
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DiyOneYuanParam {
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DiyPromotionActCollectionParam {
    /// 集合id 不传默认使用最新的大促会场集合
    pub collection_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct RangeItems {
    /// 区间的开始值
    pub range_from: Option<i64>,
    /// range_id为1表示红包抵后价（单位分）， range_id为2表示佣金比例（单位千分之几)
    pub range_id: Option<i32>,
    /// 区间的结束值
    pub range_to: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DiyRedPacketParam {
    /// 红包金额列表，200、300、500、1000、2000，单位分。红包金额和红包抵后价设置只能二选一，默认设置了红包金额会忽略红包抵后价设置
    pub amount_probability: Option<Vec<i64>>,
    /// 设置玩法，false-现金红包, true-现金券
    pub dis_text: Option<bool>,
    /// 推广页设置，false-红包开启页, true-红包领取页
    pub not_show_background: Option<bool>,
    /// 优先展示类目
    pub opt_id: Option<i32>,
    /// 自定义红包抵后价和商品佣金区间对象数组
    pub range_items: Option<Vec<RangeItems>>,
}

#[derive(Debug, Serialize)]
pub struct DiySpRedPacketParam {
    /// 商品goodsSign，支持通过goodsSign置顶落地页商品。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign: Option<String>,
    /// 商品skuId密文，支持自动选中对应sku
    pub sku_id_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TmccParam {
    /// 置顶商品的goodsSign列表
    pub goods_signs: Option<Vec<String>>,
    /// 指定活动id
    pub tmc_config_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddDdkRpPromUrlGenerateRequest {
    /// 初始金额（单位分），有效金额枚举值：300、500、700、1100和1600，默认300
    pub amount: Option<i64>,
    /// 营销工具类型，必填：-1-活动列表，0-红包(需申请推广权限)，2–新人红包，3-刮刮卡，5-员工内购，10-生成绑定备案链接，12-砸金蛋，14-千万补贴B端页面，15-充值中心B端页面，16-千万补贴C端页面，17-千万补贴投票页面，23-超级红包，24-礼金全场N折活动B端页面，27-带货赢千万，30-免单B端页面，31-免单C端页面，32-转盘得现金B端页面，33-转盘得现金C端页面，34-千万神券C端页面，35-千万神券B端页面，36-爆品日历B端页面，37-超级红包B端推品页，39-母婴馆C端页面，40-母婴馆B端页面，41-限时折扣B端页面，42-超级红包9.9C端活动页 45-大促会场集合B端页面 46-大促会场集合C端页面
    pub channel_type: Option<i32>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    pub custom_parameters: Option<String>,
    /// 一元购自定义参数，json格式，例如:{"goods_sign":"Y9b2_0uSWMFPGSaVwvfZAlm_y2ADLWZl_JQ7UYaS80K"}
    pub diy_one_yuan_param: Option<DiyOneYuanParam>,
    /// 大促会场集合页参数
    pub diy_promotion_act_collection_param: Option<DiyPromotionActCollectionParam>,
    /// 红包自定义参数，json格式
    pub diy_red_packet_param: Option<DiyRedPacketParam>,
    /// 超级红包自定义参数，json格式
    pub diy_sp_red_packet_param: Option<DiySpRedPacketParam>,
    /// 扩展参数
    pub ext_params: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// 是否生成qq小程序
    pub generate_qq_app: Option<bool>,
    /// 是否返回 schema URL
    pub generate_schema_url: Option<bool>,
    /// 是否生成微信shortLink，该字段支持超红c端活动页、超红二合一、b端推品页，单个渠道每天生成的shortLink数量有限，请合理生成shortLink链接
    pub generate_short_link: Option<bool>,
    /// 是否生成短链接。true-是，false-否，默认false
    pub generate_short_url: Option<bool>,
    /// 是否生成拼多多福利券微信小程序推广信息
    pub generate_we_app: Option<bool>,
    /// 推广位列表，长度最大为1，例如：["60005_612"]。活动页生链要求传入授权备案信息，不支持批量生链。
    pub p_id_list: Vec<String>,
    /// 刮刮卡指定金额（单位分），可指定2-100元间数值，即有效区间为：[200,10000]
    pub scratch_card_amount: Option<i64>,
    /// 千万神券C端生链扩展参数 支持置顶活动ID 和 置顶商品(品牌活动才支持)
    pub tmcc_param: Option<TmccParam>,
    /// 招商DuoID
    pub zs_duo_id: Option<i64>,
}

impl RequestType for PddDdkRpPromUrlGenerateRequest {
    type Response = PddDdkRpPromUrlGenerateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.rp.prom.url.generate"
    }
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    /// 活动描述
    #[serde(default)]
    pub desc: String,
    /// 活动地址
    #[serde(default)]
    pub url: String,
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
    /// 小程序icon
    #[serde(default)]
    pub we_app_icon_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    /// 推广移动短链接，对应出参mobile_url的短链接，与mobile_url功能一致。
    #[serde(default)]
    pub mobile_short_url: String,
    /// 推广移动链接，用户安装拼多多APP的情况下会唤起APP，否则唤起H5页面
    #[serde(default)]
    pub mobile_url: String,
    /// 推广多人团移动短链接
    #[serde(default)]
    pub multi_group_mobile_short_url: String,
    /// 推广多人团移动链接，用户安装拼多多APP的情况下会唤起APP，否则唤起H5页面
    #[serde(default)]
    pub multi_group_mobile_url: String,
    /// 推广多人团短链接
    #[serde(default)]
    pub multi_group_short_url: String,
    /// 推广多人团链接，唤起H5页面
    #[serde(default)]
    pub multi_group_url: String,
    /// qq小程序信息
    #[serde(default)]
    pub qq_app_info: Option<QqAppInfo>,
    /// schema链接，用户安装拼多多APP的情况下会唤起APP（需客户端支持schema跳转协议）
    #[serde(default)]
    pub schema_url: String,
    /// 推广短链接，对应出参url的短链接，与url功能一致
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
    /// 微信shortLink，该字段支持超红c端活动页、超红二合一、b端推品页，单个渠道每天生成的shortLink数量有限，请合理生成shortLink链接
    #[serde(default)]
    pub weixin_short_link: String,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkRpPromUrlGenerateResponse {
    /// resource_list
    #[serde(default)]
    pub resource_list: Option<Vec<Resource>>,
    /// url_list
    #[serde(default)]
    pub url_list: Option<Vec<Url>>,
}
