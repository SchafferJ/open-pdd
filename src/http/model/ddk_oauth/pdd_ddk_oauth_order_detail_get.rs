//! # 获取订单详情
//!
//! 查询订单详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthOrderDetailGetRequest {
    /// 订单号
    pub order_sn: String,
    /// 订单类型：1-推广订单；2-直播间订单
    pub query_order_type: Option<i32>,
}

impl RequestType for PddDdkOauthOrderDetailGetRequest {
    type Response = PddDdkOauthOrderDetailGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.order.detail.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthOrderDetailGetResponse {
    /// 商品活动标记数组，例：[4,7]，4-秒杀 7-百亿补贴等
    #[serde(default)]
    pub activity_tags: Vec<i32>,
    /// 多多客工具id
    #[serde(default)]
    pub auth_duo_id: i64,
    /// 结算批次号
    #[serde(default)]
    pub batch_no: String,
    /// 商品一~四级类目ID列表
    #[serde(default)]
    pub cat_ids: Vec<i64>,
    /// 是否是 cpa 新用户，1表示是，0表示否
    #[serde(default)]
    pub cpa_new: i32,
    /// CPS_Sign
    #[serde(default)]
    pub cps_sign: String,
    /// 自定义参数
    #[serde(default)]
    pub custom_parameters: String,
    /// 订单审核失败/惩罚原因
    #[serde(default)]
    pub fail_reason: String,
    /// 商品一级类目名称
    #[serde(default)]
    pub goods_category_name: String,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品价格（分）
    #[serde(default)]
    pub goods_price: i64,
    /// 商品数量
    #[serde(default)]
    pub goods_quantity: i64,
    /// goodsSign是加密后的goodsId，goodsId已下线，请使用goodsSign来替代。需要注意的是：推广链接带有goodsSign信息时，订单会返回原goodsSign；反之，会生成新的goodsSign返回。
    #[serde(default)]
    pub goods_sign: String,
    /// 商品缩略图
    #[serde(default)]
    pub goods_thumbnail_url: String,
    /// 成团编号
    #[serde(default)]
    pub group_id: i64,
    /// 是否直推 ，1表示是，0表示否
    #[serde(default)]
    pub is_direct: i32,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 店铺名称
    #[serde(default)]
    pub mall_name: String,
    /// 非补贴订单原因，例如："商品补贴达上限"，"达到单个用户下单上限"，"非指定落地页直推订单"，"订单超过2个月未审核成功"等
    #[serde(default)]
    pub no_subsidy_reason: String,
    /// 订单价格（分）
    #[serde(default)]
    pub order_amount: i64,
    /// 订单创建时间（UNIX时间戳）
    #[serde(default)]
    pub order_create_time: i64,
    /// 订单成团时间（UNIX时间戳）
    #[serde(default)]
    pub order_group_success_time: i64,
    /// 订单最后更新时间（UNIX时间戳）
    #[serde(default)]
    pub order_modify_at: i64,
    /// 订单支付时间（UNIX时间戳）
    #[serde(default)]
    pub order_pay_time: i64,
    /// 订单确认收货时间（UNIX时间戳）
    #[serde(default)]
    pub order_receive_time: i64,
    /// 订单结算时间（UNIX时间戳）
    #[serde(default)]
    pub order_settle_time: i64,
    /// 订单编号
    #[serde(default)]
    pub order_sn: String,
    /// 订单状态
    #[serde(default)]
    pub order_status: i32,
    /// 订单状态：0-已支付；1-已成团；2-确认收货；3-审核成功；4-审核失败（不可提现）；5-已经结算 ;10-已处罚
    #[serde(default)]
    pub order_status_desc: String,
    /// 订单审核时间（UNIX时间戳）
    #[serde(default)]
    pub order_verify_time: i64,
    /// 推广位id
    #[serde(default)]
    pub pid: String,
    /// 平台券金额，表示该订单使用的平台券金额，单位分
    #[serde(default)]
    pub platform_discount: i64,
    /// 打点时间
    #[serde(default)]
    pub point_time: i64,
    /// 比价状态：0：正常，1：比价
    #[serde(default)]
    pub price_compare_status: i32,
    /// 佣金（分）
    #[serde(default)]
    pub promotion_amount: i64,
    /// 佣金比例 千分比
    #[serde(default)]
    pub promotion_rate: i64,
    /// 超级红包补贴类型：0-非红包补贴订单，1-季度新用户补贴
    #[serde(default)]
    pub red_packet_type: i32,
    /// 售后状态：0：无，1：售后中，2：售后完成
    #[serde(default)]
    pub return_status: i32,
    /// 直播间订单推广duoId
    #[serde(default)]
    pub sep_duo_id: i64,
    /// 直播间推广佣金
    #[serde(default)]
    pub sep_market_fee: i32,
    /// 直播间推广自定义参数
    #[serde(default)]
    pub sep_parameters: String,
    /// 直播间订单推广位
    #[serde(default)]
    pub sep_pid: String,
    /// 直播间推广佣金比例
    #[serde(default)]
    pub sep_rate: i32,
    /// 招商分成服务费金额，单位为分
    #[serde(default)]
    pub share_amount: i32,
    /// 招商分成服务费比例，千分比
    #[serde(default)]
    pub share_rate: i32,
    /// 优势渠道专属商品补贴金额，单位为分。针对优质渠道的补贴活动，指定优势渠道可通过推广该商品获取相应补贴。补贴活动入口：[进宝网站-官方活动]
    #[serde(default)]
    pub subsidy_amount: i32,
    /// 等级补贴给渠道的收入补贴，不允许直接给下级代理展示，单位为分
    #[serde(default)]
    pub subsidy_duo_amount_level: i32,
    /// 官方活动给渠道的收入补贴金额，不允许直接给下级代理展示，单位为分
    #[serde(default)]
    pub subsidy_duo_amount_ten_million: i32,
    /// 订单补贴类型：0-非补贴订单，1-千万补贴，2-社群补贴，3-多多星选，4-品牌优选，5-千万神券
    #[serde(default)]
    pub subsidy_type: i32,
    /// 下单场景类型：0-单品推广，1-红包活动推广，4-多多进宝商城推广，7-今日爆款，8-品牌清仓，9-1.9包邮，77-刮刮卡活动推广，94-充值中心，101-品牌黑卡，103-百亿补贴频道，104-内购清单频道，105-超级红包
    #[serde(default)]
    pub r#type: i32,
    /// 链接最后一次生产时间
    #[serde(default)]
    pub url_last_generate_time: i64,
    /// 招商多多客id
    #[serde(default)]
    pub zs_duo_id: i64,
    /// 预判断是否为代购订单，-1（默认）表示未出结果，0表示预判不是代购订单，1表示代购订单，具体请以最后审核状态为准
    #[serde(default)]
    pub bandan_risk_consult: i32,
}
