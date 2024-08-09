//! 查询全站推广小时报表数据

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiReportTrHourlyReportQueryRequest {
    /// 查询日期的字符串，格式类似'2020-02-02',当前支持查询30天内数据
    #[serde(rename = "dateString")]
    pub date_string: String,
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
}

impl RequestType for PddAdApiReportTrHourlyReportQueryRequest {
    type Response = PddAdApiReportTrHourlyReportQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.report.tr.hourly.report.query"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct HourlyReport {
    /// 直接成交每笔成交金额，单位厘
    #[serde(default, rename = "avgDirectPayAmount")]
    pub avg_direct_pay_amount: f64,
    /// 间接成交每笔成交金额，单位厘
    #[serde(default, rename = "avgIndirectPayAmount")]
    pub avg_indirect_pay_amount: f64,
    /// 每笔成交金额
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
    /// 直接成交交易额，单位厘
    #[serde(default, rename = "directGmv")]
    pub direct_gmv: i64,
    /// 直接成交笔数
    #[serde(default, rename = "directOrderNum")]
    pub direct_order_num: i64,
    /// 全站费比 spend / globalGmv
    #[serde(default, rename = "globalTakeRate")]
    pub global_take_rate: f64,
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
    /// 间接成交交易额，单位厘
    #[serde(default, rename = "indirectGmv")]
    pub indirect_gmv: i64,
    /// 间接成交笔数
    #[serde(default, rename = "indirectOrderNum")]
    pub indirect_order_num: i64,
    /// 店铺收藏数
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
    /// 直接成交每笔成交金额，单位厘
    #[serde(default, rename = "avgDirectPayAmount")]
    pub avg_direct_pay_amount: f64,
    /// 间接成交每笔成交金额，单位厘
    #[serde(default, rename = "avgIndirectPayAmount")]
    pub avg_indirect_pay_amount: f64,
    /// 每笔成交金额
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
    /// 直接成交交易额，单位厘
    #[serde(default, rename = "directGmv")]
    pub direct_gmv: i64,
    /// 直接成交笔数
    #[serde(default, rename = "directOrderNum")]
    pub direct_order_num: i64,
    /// 全站费比 spend / globalGmv
    #[serde(default, rename = "globalTakeRate")]
    pub global_take_rate: f64,
    /// 广告转化支付金额，单位厘
    #[serde(default)]
    pub gmv: i64,
    /// 商品收藏数
    #[serde(default, rename = "goodsFavNum")]
    pub goods_fav_num: i64,
    /// 广告曝光量
    #[serde(default)]
    pub impression: i64,
    /// 间接成交交易额，单位厘
    #[serde(default, rename = "indirectGmv")]
    pub indirect_gmv: i64,
    /// 间接成交笔数
    #[serde(default, rename = "indirectOrderNum")]
    pub indirect_order_num: i64,
    /// 店铺收藏数
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
    /// 小时报表数据列表
    #[serde(default, rename = "hourlyReportList")]
    pub hourly_report_list: Option<Vec<HourlyReport>>,
    /// 一天的汇总数据
    #[serde(default, rename = "sumReport")]
    pub sum_report: Option<SumReport>,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiReportTrHourlyReportQueryResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
