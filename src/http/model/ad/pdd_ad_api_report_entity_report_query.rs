//! 广告主报表分级查询接口

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct QueryRange {
    /// 页数
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    /// 每页的数量
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiReportEntityReportQueryRequest {
    /// 结束日期的字符串，格式类似'2020-02-02'，当前支持查询90天内数据
    #[serde(rename = "endDateString")]
    pub end_date_string: String,
    /// entityId的维度(当前只支持0-广告主，1-计划，2-单元维度),例如根据单元查询资源位的分级数据，entityId传单元id，entityDimensionType传单元维度，queryDimensionType传资源位维度
    #[serde(rename = "entityDimensionType")]
    pub entity_dimension_type: Option<i32>,
    /// 各维度查询的主体id，查询计划维度传计划id，查询单元维度传单元id
    #[serde(rename = "entityId")]
    pub entity_id: Option<i64>,
    /// 额外的查询条件，entityDimensionType维度为单元时，须加上父级计划id（planId）的信息
    #[serde(rename = "externalParamMap")]
    pub external_param_map: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// 排序规则，0-曝光，1-点击，2-点击率，3-cpc,4-花费，5-订单量,6-gmv，7-roi,8-日期，9-cpm,10-店铺收藏，11-商品收藏
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    /// 排序顺序，0-降序，1-升序
    #[serde(rename = "orderType")]
    pub order_type: Option<i32>,
    /// 查询维度，0-广告主，1-计划，2-单元，3-定向，4-创意，5-资源位，6-关键词
    #[serde(rename = "queryDimensionType")]
    pub query_dimension_type: i32,
    /// 分页字段,不传不分页
    #[serde(rename = "queryRange")]
    pub query_range: QueryRange,
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
    /// 开始日期的字符串，格式类似'2020-02-02'，如果查询今日，startDateString和endDateString传今日的字符串，如果查询历史，startDateString和endDateString分别传开始和结束字符串，不能跨今日和历史查询
    #[serde(rename = "startDateString")]
    pub start_date_string: String,
}

impl RequestType for PddAdApiReportEntityReportQueryRequest {
    type Response = PddAdApiReportEntityReportQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.report.entity.report.query"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct EntityReport {
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
    /// 查询的维度
    #[serde(default, rename = "dimensionType")]
    pub dimension_type: i32,
    /// 查询维度的主体id
    #[serde(default, rename = "entityId")]
    pub entity_id: i64,
    /// 业务数据说明
    #[serde(default, rename = "externalFieldValues")]
    pub external_field_values: std::collections::BTreeMap<String, serde_json::Value>,
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
    /// 单页报表数据列表
    #[serde(default, rename = "entityReportList")]
    pub entity_report_list: Option<Vec<EntityReport>>,
    /// 分页数据汇总
    #[serde(default, rename = "sumReport")]
    pub sum_report: Option<SumReport>,
    /// 报表记录总数
    #[serde(default)]
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiReportEntityReportQueryResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
