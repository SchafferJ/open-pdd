//! # 创建多多礼金
//!
//! 创建多多礼金
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkCashgiftCreateRequest {
    /// 券批次领取结束时间。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub acquire_end_time: i64,
    /// 券批次领取开始时间。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub acquire_start_time: i64,
    /// 是否自动领券，默认false不自动领券
    pub auto_take: Option<bool>,
    /// 创建礼金类型：1-普通满减礼金；2-不限商品满减礼金；3-免单礼金；4-灵活面额礼金。默认为普通满减礼金
    pub cashgift_type: Option<i32>,
    /// 礼金券面额，单位为分，创建普通满减礼金、不限商品满减礼金和免单礼金时，该字段必填；创建灵活面额礼金时，该字段传空，券面额 = 商品券后价 - 期望礼金券后价，由系统自动计算
    pub coupon_amount: Option<i32>,
    /// 满减门槛，单位为分。对于普通满减礼金和不限商品满减礼金，满减门槛至少需为礼金券面额的2倍
    pub coupon_threshold_amount: Option<i32>,
    /// 使用有效期，相对时间类型为天级时，最大值为30，即领取后30天内有效；相对时间类型为小时级时，最大值为24，即领取后24小时内有效；相对时间类型为分钟级时，则最大值为60，即领取后60分钟内有效。
    pub duration: Option<i32>,
    /// 期望礼金券后价，单位为分，最小值为1。创建灵活面额礼金时必填
    pub except_amount: Option<i32>,
    /// 是否打开风控保护开关，默认false关闭
    pub fetch_risk_check: Option<bool>,
    /// 收益保护开关开启(rate_decrease_monitor = true)时必填。0-监控项发生降低；1-监控项低于礼金面额，默认为0。
    pub freeze_condition: Option<i32>,
    /// 收益保护开关开启(rate_decrease_monitor = true)时必填。0-佣金；1-补贴；2-佣金+补贴，默认为0。
    pub freeze_watch_type: Option<i32>,
    /// 是否开启全场景推广，默认false不开启全场景推广，仅支持普通满减礼金和免单礼金
    pub generate_global: Option<bool>,
    /// 商品goodsSign列表，例如：["c9r2omogKFFAc7WBwvbZU1ikIb16_J3CTa8HNN"]，最多可支持传20个商品。创建普通满减礼金、免单礼金和灵活面额礼金时，该字段必填；创建不限商品满减礼金时，该字段传空。goodsSign使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign_list: Option<Vec<String>>,
    /// 活动单链接可领券数量，默认无限制，最小值为1。
    pub link_acquire_limit: Option<i64>,
    /// 礼金名称
    pub name: Option<String>,
    /// 可使用推广位列表，例如：["60005_612"]。(列表中的PID方可推广该礼金)
    pub p_id_list: Option<Vec<String>>,
    /// 礼金券数量，创建普通满减礼金、不限商品满减礼金或免单礼金时，该字段必填；创建灵活面额礼金时，礼金券数量不固定，礼金总预算用完为止，该字段不传
    pub quantity: Option<i64>,
    /// 收益保护开关，默认false关闭，仅支持普通满减礼金和免单礼金。开启状态下，系统将根据设置内容进行监控，当监控项满足冻结条件时，系统自动冻结礼金暂停推广，防止资金损失（您可通过多多礼金状态更新接口自行恢复推广）
    pub rate_decrease_monitor: Option<bool>,
    /// 相对时间类型：1-天级；2-小时级；3-分钟级，默认为1。 例如: relative_time_type = 2,duration = 15, 表示领取后15小时内有效。
    pub relative_time_type: Option<i32>,
    /// 礼金总预算，单位为分，创建灵活面额礼金时必填。其他情况，总金额 = 礼金券数量 * 礼金券面额
    pub total_amount: Option<i64>,
    /// 单用户可领券数量，可设置范围为1~10张，默认为1张。
    pub user_limit: Option<i32>,
}

impl RequestType for PddDdkCashgiftCreateRequest {
    type Response = PddDdkCashgiftCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.cashgift.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkCashgiftCreateResponse {
    /// 礼金ID
    #[serde(default)]
    pub cash_gift_id: i64,
    /// 创建结果
    #[serde(default)]
    pub success: bool,
}
