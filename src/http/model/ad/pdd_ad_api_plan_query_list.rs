//! 查询计划列表页

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiPlanQueryListRequest {
    /// 报表数据开始日期  格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "beginDate")]
    pub begin_date: String,
    /// 报表数据截止日期 格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// 排序字段。0表示按曝光量排序，1表示按点击量排序，2表示按点击率排序，3表示按点击单价排序，4表示按消耗排序，5表示按订单数排序，6表示按交易额排序，7表示按产出比排序，8表示按日期排序，9表示按千次曝光单价排序，10表示按店铺收藏数排序，11表示按商品收藏数排序，12表示按点击转化率排序，13表示按转化成本排序，14表示按平均成交金额排序。
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
    /// 排序类型。0表示降序，1表示升序。
    #[serde(rename = "sortBy")]
    pub sort_by: Option<i32>,
}

impl RequestType for PddAdApiPlanQueryListRequest {
    type Response = PddAdApiPlanQueryListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.plan.query.list"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 计划日限额今日剩余可修改次数。一个计划一天可修改五次日限额。
    #[serde(default, rename = "availableBudgetChangeNumberToday")]
    pub available_budget_change_number_today: i32,
    /// 每笔成交金额(average pay amount)，单位厘
    #[serde(default, rename = "avgPayAmount")]
    pub avg_pay_amount: f64,
    /// 计划日限额今日已修改次数
    #[serde(default, rename = "budgetChangedNumberToday")]
    pub budget_changed_number_today: i32,
    /// 广告点击量
    #[serde(default)]
    pub click: i64,
    /// 平均点击花费，单位厘
    #[serde(default)]
    pub cpc: f64,
    /// 千次展现成本
    #[serde(default)]
    pub cpm: f64,
    /// 广告点击率
    #[serde(default)]
    pub ctr: f64,
    /// 点击转化率
    #[serde(default)]
    pub cvr: f64,
    /// 分时折扣比例。千分比。
    #[serde(default, rename = "discountRate")]
    pub discount_rate: i32,
    /// 广告转化支付金额，单位厘
    #[serde(default)]
    pub gmv: i64,
    /// 商品收藏数
    #[serde(default, rename = "goodsFavNum")]
    pub goods_fav_num: i64,
    /// 广告曝光量
    #[serde(default)]
    pub impression: i64,
    /// 店铺关注数
    #[serde(default, rename = "mallFavNum")]
    pub mall_fav_num: i64,
    /// 日消耗上限。单位厘
    #[serde(default, rename = "maxCost")]
    pub max_cost: i64,
    /// 广告主账户状态。1表示余额充足，2表示余额不足，3表示超出消耗上限。
    #[serde(default, rename = "merchantAccountStatus")]
    pub merchant_account_status: i32,
    /// 操作状态。1表示开启，2表示暂停。
    #[serde(default, rename = "merchantOperateStatus")]
    pub merchant_operate_status: i32,
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 计划ID
    #[serde(default, rename = "planId")]
    pub plan_id: i64,
    /// 计划名称
    #[serde(default, rename = "planName")]
    pub plan_name: String,
    /// 推广策略。1表示自定义推广，2表示智能推广。
    #[serde(default, rename = "planStrategy")]
    pub plan_strategy: i32,
    /// 广告惩罚原因
    #[serde(default, rename = "punishReason")]
    pub punish_reason: String,
    /// 广告投入产出比
    #[serde(default)]
    pub roi: f64,
    /// 广告消耗,单位厘
    #[serde(default)]
    pub spend: i64,
    /// 单元状态。1表示推广中，2表示手动暂停，3表示余额不足，4表示到达日限额，5表示无推广中单元，6表示已删除，7表示系统暂停-投放已结束，8表示系统暂停-投放未开始，9表示不在投放时段，10表示待确认推广单，11表示推广费用待锁定，12表示合约已失效，13表示限制推广，14表示部分推广中，15表示店铺限制推广。
    #[serde(default)]
    pub status: i32,
    /// 转化成本
    #[serde(default, rename = "transactionCost")]
    pub transaction_cost: f64,
    /// 计划下的单元数量
    #[serde(default, rename = "unitNum")]
    pub unit_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiPlanQueryListResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
