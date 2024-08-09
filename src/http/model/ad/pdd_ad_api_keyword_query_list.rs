//! 查询关键词列表

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiKeywordQueryListRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 报表数据开始日期
    #[serde(rename = "beginDate")]
    pub begin_date: String,
    /// 报表数据截止日期
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// 排序字段。0表示按曝光量排序，1表示按点击量排序，2表示按点击率排序，3表示按点击单价排序，4表示按消耗排序，5表示按订单数排序，6表示按交易额排序，7表示按产出比排序，8表示按日期排序，9表示按千次曝光单价排序，10表示按店铺收藏数排序，11表示按商品收藏数排序，12表示按点击转化率排序，13表示按转化成本排序，14表示按平均成交金额排序。
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    /// 排序类型。0表示降序，1表示升序。
    #[serde(rename = "sortBy")]
    pub sort_by: Option<i32>,
}

impl RequestType for PddAdApiKeywordQueryListRequest {
    type Response = PddAdApiKeywordQueryListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.keyword.query.list"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PremiumReportData {
    /// 每笔成交金额(average pay amount)，单位厘
    #[serde(default, rename = "avgPayAmount")]
    pub avg_pay_amount: f64,
    /// 广告点击量
    #[serde(default)]
    pub click: i64,
    /// 平均点击花费，单位厘
    #[serde(default)]
    pub cpc: f64,
    /// er 千次展现成本
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
    /// 平均推广位置
    #[serde(default, rename = "keywordAdIdx")]
    pub keyword_ad_idx: f64,
    /// 店铺收藏数
    #[serde(default, rename = "mallFavNum")]
    pub mall_fav_num: i64,
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 广告排名（平均数）
    #[serde(default, rename = "rankAverage")]
    pub rank_average: f64,
    /// 广告排名（中位数）
    #[serde(default, rename = "rankMedian")]
    pub rank_median: f64,
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
    /// 每笔成交金额(average pay amount)，单位厘
    #[serde(default, rename = "avgPayAmount")]
    pub avg_pay_amount: f64,
    /// 出价
    #[serde(default)]
    pub bid: i64,
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
    /// 曝光状态。1表示正常，2表示15天之内无展现。
    #[serde(default, rename = "impressionStatus")]
    pub impression_status: i32,
    /// 平均推广位置
    #[serde(default, rename = "keywordAdIdx")]
    pub keyword_ad_idx: f64,
    /// 关键词Id
    #[serde(default, rename = "keywordId")]
    pub keyword_id: i64,
    /// 店铺收藏数
    #[serde(default, rename = "mallFavNum")]
    pub mall_fav_num: i64,
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 溢价比例。万分比
    #[serde(default, rename = "premiumRate")]
    pub premium_rate: i64,
    /// 溢价报表
    #[serde(default, rename = "premiumReportData")]
    pub premium_report_data: Option<PremiumReportData>,
    /// 质量分
    #[serde(default, rename = "qualityScore")]
    pub quality_score: i32,
    /// 广告排名（平均数）
    #[serde(default, rename = "rankAverage")]
    pub rank_average: f64,
    /// 广告排名（中位数）
    #[serde(default, rename = "rankMedian")]
    pub rank_median: f64,
    /// 广告投入产出比
    #[serde(default)]
    pub roi: f64,
    /// 广告消耗,单位厘
    #[serde(default)]
    pub spend: i64,
    /// 关键词状态。1表示推广中，2表示已删除。
    #[serde(default)]
    pub status: i32,
    /// 转化成本
    #[serde(default, rename = "transactionCost")]
    pub transaction_cost: f64,
    /// 关键词
    #[serde(default)]
    pub word: String,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiKeywordQueryListResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
