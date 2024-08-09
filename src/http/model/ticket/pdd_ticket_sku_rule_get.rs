//! # 旅游门票商品履约生效规则查询
//!
//! 商品履约生效规则查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddTicketSkuRuleGetRequest {
    /// 商户履约规则 id
    pub out_rule_id: Option<String>,
    /// 上传商品的上传序列 ID
    pub rule_id: Option<String>,
}

impl RequestType for PddTicketSkuRuleGetRequest {
    type Response = PddTicketSkuRuleGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.sku.rule.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct BookerInfoLimitation {
    /// 需要下单人信息
    #[serde(default)]
    pub booker_required: i32,
    /// 下单人手机
    #[serde(default)]
    pub mobile: i32,
}

#[derive(Debug, Deserialize)]
pub struct EnterTime {
    /// 备注
    #[serde(default)]
    pub comment: String,
    /// 入园结束时间
    #[serde(default)]
    pub end_at: String,
    /// 入园开始时间
    #[serde(default)]
    pub start_at: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketTime {
    /// 备注
    #[serde(default)]
    pub comment: String,
    /// 换票结束时间
    #[serde(default)]
    pub end_at: String,
    /// 换票开始时间
    #[serde(default)]
    pub start_at: String,
}

#[derive(Debug, Deserialize)]
pub struct BookingNotice {
    /// 入园地址
    #[serde(default)]
    pub enter_address: String,
    /// 入园时间
    #[serde(default)]
    pub enter_time: Option<Vec<EnterTime>>,
    /// 入园方式
    #[serde(default)]
    pub enter_ways: String,
    /// 补充说明
    #[serde(default)]
    pub extra_desc: String,
    /// 费用包含
    #[serde(default)]
    pub fee_include: String,
    /// 费用不包含
    #[serde(default)]
    pub fee_not_include: String,
    /// 重要提示
    #[serde(default)]
    pub important_notice: String,
    /// 通关限制时间
    #[serde(default)]
    pub pass_time_limit: i32,
    /// 换票地址
    #[serde(default)]
    pub ticket_place: String,
    /// 换票时间
    #[serde(default)]
    pub ticket_time: Option<Vec<TicketTime>>,
}

#[derive(Debug, Deserialize)]
pub struct OrderLimitation {
    /// 周期长度
    #[serde(default)]
    pub cycle_length: i32,
    /// 限制类型
    #[serde(default)]
    pub limitation_type: i32,
    /// 周期类型
    #[serde(default)]
    pub limit_cycle: i32,
    /// 限购数量
    #[serde(default)]
    pub limit_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct ProviderBusinessHour {
    /// 结束时间
    #[serde(default)]
    pub close_at: String,
    /// 开始时间
    #[serde(default)]
    pub open_at: String,
    /// 描述
    #[serde(default)]
    pub time_info: String,
}

#[derive(Debug, Deserialize)]
pub struct ProviderContactInfo {
    /// 服务时间
    #[serde(default)]
    pub provider_business_hour: Option<Vec<ProviderBusinessHour>>,
    /// 服务商名称
    #[serde(default)]
    pub provider_name: String,
    /// 服务商联系电话
    #[serde(default)]
    pub provider_telephone: String,
}

#[derive(Debug, Deserialize)]
pub struct RefundRules {
    /// 游玩日 0 点提前 或之后分钟数
    #[serde(default)]
    pub ahead_time: i32,
    /// 扣费值
    #[serde(default)]
    pub deduction_fee: i32,
    /// 费率单位
    #[serde(default)]
    pub deduction_unit: i32,
    /// 规则类型
    #[serde(default)]
    pub r#type: i32,
}

#[derive(Debug, Deserialize)]
pub struct RefundLimitations {
    /// 是否可退
    #[serde(default)]
    pub is_refundable: i32,
    /// 退款规则
    #[serde(default)]
    pub refund_rules: Option<Vec<RefundRules>>,
}

#[derive(Debug, Deserialize)]
pub struct TravelerInfoLimitation {
    /// 游玩人证件
    #[serde(default)]
    pub credential: i32,
    /// 游玩人名字
    #[serde(default)]
    pub name: i32,
    /// 出游人信息设置
    #[serde(default)]
    pub traveler_required: i32,
}

#[derive(Debug, Deserialize)]
pub struct ValidLimitation {
    /// 天数内有效
    #[serde(default)]
    pub days_time: i32,
    /// 结束时间
    #[serde(default)]
    pub end_time: i64,
    /// 开始时间
    #[serde(default)]
    pub start_time: i64,
    /// 有效期时间类型
    #[serde(default)]
    pub time_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddTicketSkuRuleGetResponse {
    /// 下单人信息设置
    #[serde(default)]
    pub booker_info_limitation: Option<BookerInfoLimitation>,
    /// 预定须知
    #[serde(default)]
    pub booking_notice: Option<BookingNotice>,
    /// 下单限制
    #[serde(default)]
    pub order_limitation: Option<OrderLimitation>,
    /// 商户rule ID
    #[serde(default)]
    pub out_rule_id: String,
    /// 服务商联系方式
    #[serde(default)]
    pub provider_contact_info: Option<ProviderContactInfo>,
    /// 退款规则
    #[serde(default)]
    pub refund_limitations: Option<RefundLimitations>,
    /// 拼多多 rule ID
    #[serde(default)]
    pub rule_id: String,
    /// 商户rule 名称
    #[serde(default)]
    pub rule_name: String,
    /// 规则版本
    #[serde(default)]
    pub rule_version: String,
    /// 游玩人信息
    #[serde(default)]
    pub traveler_info_limitation: Option<TravelerInfoLimitation>,
    /// 卡券有效期设置
    #[serde(default)]
    pub valid_limitation: Option<ValidLimitation>,
}
