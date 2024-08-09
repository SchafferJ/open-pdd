//! 查询单元列表页

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitQueryListRequest {
    /// 报表数据开始日期 格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "beginDate")]
    pub begin_date: String,
    /// 报表数据截止日期 格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// 排序字段。0表示按曝光量排序，1表示按点击量排序，2表示按点击率排序，3表示按点击单价排序，4表示按消耗排序，5表示按订单数排序，6表示按交易额排序，7表示按产出比排序，8表示按日期排序，9表示按千次曝光单价排序，10表示按店铺收藏数排序，11表示按商品收藏数排序，12表示按点击转化率排序，13表示按转化成本排序，14表示按平均成交金额排序。
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    /// 广告计划ID。
    #[serde(rename = "planId")]
    pub plan_id: i64,
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
    /// 排序类型。0表示降序，1表示升序。
    #[serde(rename = "sortBy")]
    pub sort_by: Option<i32>,
}

impl RequestType for PddAdApiUnitQueryListRequest {
    type Response = PddAdApiUnitQueryListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.query.list"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct OptionalOptimizationBidOutputMessage {
    /// 可选优化目标出价
    #[serde(default, rename = "optimizationBid")]
    pub optimization_bid: i64,
    /// 可选优化目标
    #[serde(default, rename = "optimizationGoal")]
    pub optimization_goal: i32,
}

#[derive(Debug, Deserialize)]
pub struct OptimizationOutputMessage {
    /// 优化出价
    #[serde(default, rename = "optimizationBid")]
    pub optimization_bid: i64,
    /// 优化启用时间。启用优化时未必立即开始，取决于数据积累状态。
    #[serde(default, rename = "optimizationEnableTime")]
    pub optimization_enable_time: String,
    /// 优化目标。0表示不优化。1表示优化ROI，2表示优化转化成本。
    #[serde(default, rename = "optimizationGoal")]
    pub optimization_goal: i32,
    /// 优化方式。0表示不优化，1表示ECPC，2表示OCPC。
    #[serde(default, rename = "optimizationMethod")]
    pub optimization_method: i32,
    /// 优化开始时间
    #[serde(default, rename = "optimizationStartTime")]
    pub optimization_start_time: String,
    /// 可选优化目标信息列表
    #[serde(default, rename = "optionalOptimizationBidOutputMessageList")]
    pub optional_optimization_bid_output_message_list: Option<Vec<OptionalOptimizationBidOutputMessage>>,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 每笔成交金额(average pay amount)，单位厘
    #[serde(default, rename = "avgPayAmount")]
    pub avg_pay_amount: f64,
    /// 广告单元出价。单位厘
    #[serde(default)]
    pub bid: i64,
    /// 类目ID
    #[serde(default, rename = "catId")]
    pub cat_id: i64,
    /// 类目名称
    #[serde(default, rename = "catName")]
    pub cat_name: String,
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
    /// 智能推广数据积累状态。0表示未知，1表示智能投放期，2表示数据积累期，3表示数据积累缓慢。
    #[serde(default, rename = "dataAccumulationStatus")]
    pub data_accumulation_status: i32,
    /// 操作状态。0表示未知，1表示启用，2表示暂停。
    #[serde(default, rename = "dataOperateStatus")]
    pub data_operate_status: i32,
    /// 广告转化支付金额，单位厘
    #[serde(default)]
    pub gmv: i64,
    /// 商品收藏数
    #[serde(default, rename = "goodsFavNum")]
    pub goods_fav_num: i64,
    /// 商品ID
    #[serde(default, rename = "goodsId")]
    pub goods_id: i64,
    /// 商品名称
    #[serde(default, rename = "goodsName")]
    pub goods_name: String,
    /// 广告曝光量
    #[serde(default)]
    pub impression: i64,
    /// 店铺关注数
    #[serde(default, rename = "mallFavNum")]
    pub mall_fav_num: i64,
    /// 广告主ID
    #[serde(default, rename = "mallId")]
    pub mall_id: i64,
    /// 最大团购价。单位厘
    #[serde(default, rename = "maxGroupPrice")]
    pub max_group_price: i64,
    /// 最小团购价。单位厘
    #[serde(default, rename = "minGroupPrice")]
    pub min_group_price: i64,
    /// 优化目标。0表示不优化。1表示优化ROI，2表示优化转化成本。
    #[serde(default, rename = "optimizationGoal")]
    pub optimization_goal: i32,
    /// 智能优化广告相关
    #[serde(default, rename = "optimizationOutputMessage")]
    pub optimization_output_message: Option<OptimizationOutputMessage>,
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 广告计划ID
    #[serde(default, rename = "planId")]
    pub plan_id: i64,
    /// 广告计划名称
    #[serde(default, rename = "planName")]
    pub plan_name: String,
    /// 推广策略。0表示未知，1表示自定义推广，2表示智能推广。
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
    /// 单元状态。1表示推广中，2表示手动暂停，3表示商品售罄，4表示商品下架，5表示限制推广，6表示已删除，7表示审核中，8表示无推广中创意，9表示审核驳回。
    #[serde(default)]
    pub status: i32,
    /// 商品图
    #[serde(default, rename = "thumbUrl")]
    pub thumb_url: String,
    /// 转化成本
    #[serde(default, rename = "transactionCost")]
    pub transaction_cost: f64,
    /// 广告单元ID
    #[serde(default, rename = "unitId")]
    pub unit_id: i64,
    /// 广告单元名称
    #[serde(default, rename = "unitName")]
    pub unit_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitQueryListResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
