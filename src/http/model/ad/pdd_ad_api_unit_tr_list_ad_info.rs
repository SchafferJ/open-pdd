//! 查询全站推广广告信息

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrListAdInfoRequest {
    /// 报表结束日期 格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// 商品id列表
    #[serde(rename = "goodsIds")]
    pub goods_ids: Option<Vec<i64>>,
    /// 排序字段，支持报表字段枚举：0-曝光，1-点击，2-点击率，3-cpc,4-花费，5-订单量,6-gmv，7-roi,8-日期，9-cpm,10-店铺收藏，11-商品收藏
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    /// 排序类型,0-倒序，1-正序
    #[serde(rename = "sortBy")]
    pub sort_by: Option<i32>,
    /// 报表开始日期 格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// 分页大小 默认10
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// 分页页数 默认1
    #[serde(rename = "pageNumber")]
    pub page_number: Option<i32>,
}

impl RequestType for PddAdApiUnitTrListAdInfoRequest {
    type Response = PddAdApiUnitTrListAdInfoResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.list.ad.info"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct ReportInfo {
    /// 直接成交每笔成交金额，单位厘
    #[serde(default, rename = "avgDirectPayAmount")]
    pub avg_direct_pay_amount: f64,
    /// 间接成交每笔成交金额，单位厘
    #[serde(default, rename = "avgIndirectPayAmount")]
    pub avg_indirect_pay_amount: f64,
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
    /// 广告 UV
    #[serde(default, rename = "uniqueView")]
    pub unique_view: i64,
}

#[derive(Debug, Deserialize)]
pub struct AdInfos {
    /// 单元名称
    #[serde(default, rename = "adName")]
    pub ad_name: String,
    /// 广告状态
    #[serde(default, rename = "adStatus")]
    pub ad_status: i32,
    /// 审核驳回原因
    #[serde(default, rename = "auditRefusedReason")]
    pub audit_refused_reason: String,
    /// 出价类型 1-目标roi 2-成交出价
    #[serde(default, rename = "bidType")]
    pub bid_type: i32,
    /// 数据积累期
    #[serde(default, rename = "dataAccumulationStatus")]
    pub data_accumulation_status: i32,
    /// 计划状态
    #[serde(default, rename = "dataOperateStatus")]
    pub data_operate_status: i32,
    /// 商品id
    #[serde(default, rename = "goodsId")]
    pub goods_id: i64,
    /// 商品名称
    #[serde(default, rename = "goodsName")]
    pub goods_name: String,
    /// 店铺id
    #[serde(default, rename = "mallId")]
    pub mall_id: i64,
    /// 日限额
    #[serde(default, rename = "maxCost")]
    pub max_cost: i64,
    /// 最小团购价
    #[serde(default, rename = "minGroupPrice")]
    pub min_group_price: i64,
    /// 成交出价
    #[serde(default, rename = "optimizationBid")]
    pub optimization_bid: i64,
    /// 广告报表信息
    #[serde(default, rename = "reportInfo")]
    pub report_info: Option<ReportInfo>,
    /// 广告限制原因
    #[serde(default, rename = "restrictionReason")]
    pub restriction_reason: String,
    /// 目标roi
    #[serde(default, rename = "targetRoi")]
    pub target_roi: i64,
    /// 商品图片
    #[serde(default, rename = "thumbUrl")]
    pub thumb_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 商品全站推广信息列表
    #[serde(default, rename = "adInfos")]
    pub ad_infos: Option<Vec<AdInfos>>,
    /// 列表总数
    #[serde(default, rename = "totalAdNum")]
    pub total_ad_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrListAdInfoResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
