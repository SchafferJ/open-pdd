//! 报表小时数据查询接口

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiReportHourlyReportQueryRequest {
    /// 查询日期的字符串，格式类似'2020-02-02',当前支持查询30天内数据
    #[serde(rename = "dateString")]
    pub date_string: String,
    /// 各维度查询的主体id，查询计划维度传计划id，查询单元维度传单元id
    #[serde(rename = "entityId")]
    pub entity_id: Option<i64>,
    /// 查询维度，0-广告主，1-计划，2-单元,当前只支持到单元维度
    #[serde(rename = "queryDimensionType")]
    pub query_dimension_type: i32,
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiReportHourlyReportQueryRequest {
    type Response = PddAdApiReportHourlyReportQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.report.hourly.report.query"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct HourlyReport {
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
    /// 小时：0 ~ 23
    #[serde(default)]
    pub hour: i32,
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
    #[serde(default, rename = "hourlyReportList")]
    pub hourly_report_list: Option<Vec<HourlyReport>>,
    #[serde(default, rename = "sumReport")]
    pub sum_report: Option<SumReport>,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiReportHourlyReportQueryResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
