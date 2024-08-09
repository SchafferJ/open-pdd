//! # 旅游门票商品履约规则新增
//!
//! 供应商新增商品规则
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BookerInfoLimitation {
    /// 需要下单人信息
    pub booker_required: i32,
    /// 下单人手机
    pub mobile: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct EnterTime {
    /// 备注
    pub comment: Option<String>,
    /// 入园结束时间
    pub end_at: String,
    /// 入园开始时间
    pub start_at: String,
}

#[derive(Debug, Serialize)]
pub struct TicketTime {
    /// 备注
    pub comment: Option<String>,
    /// 换票结束时间
    pub end_at: Option<String>,
    /// 换票开始时间
    pub start_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BookingNotice {
    /// 入园地址
    pub enter_address: String,
    /// 入园时间
    pub enter_time: Option<Vec<EnterTime>>,
    /// 入园方式
    pub enter_ways: Option<String>,
    /// 补充说明
    pub extra_desc: Option<String>,
    /// 费用包含
    pub fee_include: Option<String>,
    /// 费用不包含
    pub fee_not_include: Option<String>,
    /// 重要提示
    pub important_notice: Option<String>,
    /// 通关限制时间
    pub pass_time_limit: Option<i32>,
    /// 换票地址
    pub ticket_place: String,
    /// 换票时间
    pub ticket_time: Option<Vec<TicketTime>>,
}

#[derive(Debug, Serialize)]
pub struct OrderLimitation {
    /// 周期长度
    pub cycle_length: Option<i32>,
    /// 限制类型
    pub limitation_type: Option<i32>,
    /// 周期类型
    pub limit_cycle: Option<i32>,
    /// 限购数量
    pub limit_num: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProviderBusinessHour {
    /// 结束时间
    pub close_at: Option<String>,
    /// 开始时间
    pub open_at: Option<String>,
    /// 描述
    pub time_info: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProviderContactInfo {
    /// 服务时间
    pub provider_business_hour: Option<Vec<ProviderBusinessHour>>,
    /// 服务商名称
    pub provider_name: String,
    /// 服务商联系电话
    pub provider_telephone: String,
}

#[derive(Debug, Serialize)]
pub struct RefundRules {
    /// 游玩日 0 点提前 或之后分钟数
    pub ahead_time: Option<i32>,
    /// 扣费值
    pub deduction_fee: i32,
    /// 费率单位
    pub deduction_unit: i32,
    /// 规则类型
    pub r#type: i32,
}

#[derive(Debug, Serialize)]
pub struct RefundLimitations {
    /// 是否可退
    pub is_refundable: i32,
    /// 退款规则
    pub refund_rules: Option<Vec<RefundRules>>,
}

#[derive(Debug, Serialize)]
pub struct TravelerInfoLimitation {
    /// 游玩人证件
    pub credential: Option<i32>,
    /// 游玩人名字
    pub name: Option<i32>,
    /// 出游人信息设置
    pub traveler_required: i32,
}

#[derive(Debug, Serialize)]
pub struct ValidLimitation {
    /// 天数内有效
    pub days_time: Option<i32>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 开始时间
    pub start_time: Option<i64>,
    /// 有效期时间类型
    pub time_type: i32,
}

#[derive(Debug, Serialize)]
pub struct PddTicketSkuRuleAddRequest {
    /// 下单人信息设置
    pub booker_info_limitation: BookerInfoLimitation,
    /// 预定须知
    pub booking_notice: BookingNotice,
    /// 下单限制
    pub order_limitation: Option<OrderLimitation>,
    /// 商户rule ID
    pub out_rule_id: Option<String>,
    /// 服务商联系方式
    pub provider_contact_info: ProviderContactInfo,
    /// 退款规则
    pub refund_limitations: RefundLimitations,
    /// 商户rule 名称
    pub rule_name: String,
    /// 游玩人信息
    pub traveler_info_limitation: TravelerInfoLimitation,
    /// 卡券有效期设置
    pub valid_limitation: ValidLimitation,
}

impl RequestType for PddTicketSkuRuleAddRequest {
    type Response = PddTicketSkuRuleAddResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.sku.rule.add"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTicketSkuRuleAddResponse {
    /// 规则 ID
    #[serde(default)]
    pub rule_id: String,
    /// 版本
    #[serde(default)]
    pub rule_version: String,
}
