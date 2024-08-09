//! 查询活动报表信息

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiReportActivityReportQueryRequest {
    /// 结束日期的字符串，格式类似'2020-02-02'
    #[serde(rename = "endDateString")]
    pub end_date_string: String,
    /// 场景类型：3联合推广。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
    /// 开始日期的字符串，格式类似'2020-02-02'，如果查询今日，startDateString和endDateString传今日的字符串，如果查询历史，startDateString和endDateString分别传开始和结束字符串，不能跨今日和历史查询
    #[serde(rename = "startDateString")]
    pub start_date_string: String,
}

impl RequestType for PddAdApiReportActivityReportQueryRequest {
    type Response = PddAdApiReportActivityReportQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.report.activity.report.query"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Activities {
    /// 活动开始时间,日期格式:yyyy-MM-dd HH:mm
    #[serde(default, rename = "beginDate")]
    pub begin_date: String,
    /// 活动结束日期,日期格式:yyyy-MM-dd HH:mm
    #[serde(default, rename = "endDate")]
    pub end_date: String,
    /// 活动名称
    #[serde(default)]
    pub name: String,
    /// 活动报名截止时间,日期格式:yyyy-MM-dd HH:mm
    #[serde(default, rename = "signUpDeadline")]
    pub sign_up_deadline: String,
    /// 本场活动花费（单位厘）
    #[serde(default)]
    pub spend: i64,
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
    /// 活动列表
    #[serde(default)]
    pub activities: Option<Vec<Activities>>,
    /// 汇总报表
    #[serde(default, rename = "sumReport")]
    pub sum_report: Option<SumReport>,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiReportActivityReportQueryResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
