//! 广告主分天报表查询

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiReportDailyReportQueryRequest {
    /// 结束日期的字符串，格式类似'2020-02-02'，当前支持查询90天内数据
    #[serde(rename = "endDateString")]
    pub end_date_string: String,
    /// 各维度查询的主体id，查询计划维度传计划id，查询单元维度传单元id，查询关键词维度传关键词id，查询创意传创意id，查询广告主维度，资源位和定向维度不用传此参数
    #[serde(rename = "entityId")]
    pub entity_id: Option<i64>,
    /// 额外的查询条件，查询关键词，创意维度的的时候要在此传单元id(adId)的信息，查询资源位的时候要传单元id(adId）和资源位类型(bidReferenceId),查询定向维度要传单元id(adId),定向类型(bidReferenceId)，如果是查二级定向，需要传二级定向id(subBidReferenceId)
    #[serde(rename = "externalParamMap")]
    pub external_param_map: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// 查询维度，0-广告主，1-计划，2-单元，3-定向，4-创意，5-资源位，6-关键词
    #[serde(rename = "queryDimensionType")]
    pub query_dimension_type: i32,
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
    /// 开始日期的字符串，格式类似'2020-02-02'，如果查询今日，startDateString和endDateString传今日的字符串，如果查询历史，startDateString和endDateString分别传开始和结束字符串，不能跨今日和历史查询
    #[serde(rename = "startDateString")]
    pub start_date_string: String,
}

impl RequestType for PddAdApiReportDailyReportQueryRequest {
    type Response = PddAdApiReportDailyReportQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.report.daily.report.query"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct DailyReport {
    /// 每笔成交金额(average pay amount)，单位厘
    #[serde(default, rename = "avgPayAmount")]
    pub avg_pay_amount: f64,
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
    /// 日期
    #[serde(default)]
    pub date: String,
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
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 广告投入产出比
    #[serde(default)]
    pub roi: f64,
    /// 广告消耗,单位厘
    #[serde(default)]
    pub spend: i64,
    /// 转化成本
    #[serde(default, rename = "transactionCost")]
    pub transaction_cost: f64,
}

#[derive(Debug, Deserialize)]
pub struct SumReport {
    /// 每笔成交金额(average pay amount)，单位厘
    #[serde(default, rename = "avgPayAmount")]
    pub avg_pay_amount: f64,
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
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 广告投入产出比
    #[serde(default)]
    pub roi: f64,
    /// 广告消耗,单位厘
    #[serde(default)]
    pub spend: i64,
    /// 转化成本
    #[serde(default, rename = "transactionCost")]
    pub transaction_cost: f64,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 分天报表数据列表
    #[serde(default, rename = "dailyReportList")]
    pub daily_report_list: Option<Vec<DailyReport>>,
    /// 分天报表数据汇总
    #[serde(default, rename = "sumReport")]
    pub sum_report: Option<SumReport>,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiReportDailyReportQueryResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
