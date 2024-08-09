//! # 最后更新时间段增量同步推广订单信息
//!
//! 按照时间段获取授权多多客下面所有多多客的推广订单信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOrderListIncrementGetRequest {
    /// 是否为礼金订单，查询礼金订单时，订单类型不填（默认推广订单）。
    pub cash_gift_order: Option<bool>,
    /// 查询结束时间，和开始时间相差不能超过24小时。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub end_update_time: i64,
    /// 第几页，从1到10000，默认1，注：使用最后更新时间范围增量同步时，必须采用倒序的分页方式（从最后一页往回取）才能避免漏单问题。
    pub page: Option<i32>,
    /// 返回的每页结果订单数，默认为100，范围为10到100，建议使用40~50，可以提高成功率，减少超时数量。
    pub page_size: Option<i32>,
    /// 订单类型：1-推广订单；2-直播间订单
    pub query_order_type: Option<i32>,
    /// 是否返回总数，默认为true，如果指定false, 则返回的结果中不包含总记录数，通过此种方式获取增量数据，效率在原有的基础上有80%的提升。
    pub return_count: Option<bool>,
    /// 最近90天内多多进宝商品订单更新时间--查询时间开始。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub start_update_time: i64,
}

impl RequestType for PddDdkOrderListIncrementGetRequest {
    type Response = PddDdkOrderListIncrementGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.order.list.increment.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Order {
    /// 商品活动标记数组，例：[4,7]，4-秒杀 7-百亿补贴等
    #[serde(default)]
    pub activity_tags: Vec<i32>,
    /// 多多客工具id
    #[serde(default)]
    pub auth_duo_id: i64,
    /// 预判断是否为代购订单，-1（默认）表示未出结果，0表示预判不是代购订单，1表示代购订单，具体请以最后审核状态为准
    #[serde(default)]
    pub bandan_risk_consult: i32,
    /// 结算批次号
    #[serde(default)]
    pub batch_no: String,
    /// 订单关联礼金活动Id
    #[serde(default)]
    pub cash_gift_id: i64,
    /// 商品一~四级类目ID列表
    #[serde(default)]
    pub cat_ids: Vec<i64>,
    /// 是否是 cpa 新用户，1表示是，0表示否
    #[serde(default)]
    pub cpa_new: i32,
    /// 自定义参数
    #[serde(default)]
    pub custom_parameters: String,
    /// 订单审核失败/惩罚原因
    #[serde(default)]
    pub fail_reason: String,
    /// 商品一级类目名称
    #[serde(default)]
    pub goods_category_name: String,
    /// 商品ID
    #[serde(default)]
    pub goods_id: i64,
    /// 商品标题
    #[serde(default)]
    pub goods_name: String,
    /// 订单中sku的单件价格，单位为分
    #[serde(default)]
    pub goods_price: i64,
    /// 购买商品的数量
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
    /// 计入千万补贴额度(仅top渠道享受) 值为0时不计入 其他为null
    #[serde(default)]
    pub in_ten_million_subsidy_quota: i32,
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
    /// 不计入千万补贴额度原因
    #[serde(default)]
    pub not_in_ten_million_subsidy_quota_reason: String,
    /// 实际支付金额，单位为分
    #[serde(default)]
    pub order_amount: i64,
    /// 订单生成时间，UNIX时间戳
    #[serde(default)]
    pub order_create_time: i64,
    /// 成团时间
    #[serde(default)]
    pub order_group_success_time: i64,
    /// 订单ID
    #[serde(default)]
    pub order_id: String,
    /// 最后更新时间
    #[serde(default)]
    pub order_modify_at: i64,
    /// 支付时间
    #[serde(default)]
    pub order_pay_time: i64,
    /// 确认收货时间
    #[serde(default)]
    pub order_receive_time: i64,
    /// 结算时间
    #[serde(default)]
    pub order_settle_time: i64,
    /// 推广订单编号
    #[serde(default)]
    pub order_sn: String,
    /// 订单状态：0-已支付；1-已成团；2-确认收货；3-审核成功；4-审核失败（不可提现）；5-已经结算 ;10-已处罚
    #[serde(default)]
    pub order_status: i32,
    /// 订单状态描述
    #[serde(default)]
    pub order_status_desc: String,
    /// 审核时间
    #[serde(default)]
    pub order_verify_time: i64,
    /// 推广位ID
    #[serde(default)]
    pub p_id: String,
    /// 平台券金额，表示该订单使用的平台券金额，单位分
    #[serde(default)]
    pub platform_discount: i64,
    /// 比价状态：0：正常，1：比价
    #[serde(default)]
    pub price_compare_status: i32,
    /// 佣金金额，单位为分
    #[serde(default)]
    pub promotion_amount: i64,
    /// 佣金比例，千分比
    #[serde(default)]
    pub promotion_rate: i64,
    /// 超级红包补贴类型：0-非红包补贴订单，1-季度新用户补贴
    #[serde(default)]
    pub red_packet_type: i32,
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
    /// 补贴订单备注
    #[serde(default)]
    pub subsidy_order_remark: String,
    /// 订单补贴类型：0-非补贴订单，1-千万补贴，2-社群补贴，3-多多星选，4-品牌优选，5-千万神券
    #[serde(default)]
    pub subsidy_type: i32,
    /// 下单场景类型：0-单品推广，1-红包活动推广，4-多多进宝商城推广，7-今日爆款，8-品牌清仓，9-1.9包邮，77-刮刮卡活动推广，94-充值中心，101-品牌黑卡，103-百亿补贴频道，104-内购清单频道，105-超级红包
    #[serde(default)]
    pub r#type: i32,
    /// 招商多多客id
    #[serde(default)]
    pub zs_duo_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOrderListIncrementGetResponse {
    /// 多多进宝推广位对象列表
    #[serde(default)]
    pub order_list: Option<Vec<Order>>,
    /// 请求到的结果数
    #[serde(default)]
    pub total_count: i64,
}
