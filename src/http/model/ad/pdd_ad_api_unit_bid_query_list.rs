//! 查询定向/资源位列表

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitBidQueryListRequest {
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: i64,
    /// 报表数据开始日期
    #[serde(rename = "beginDate")]
    pub begin_date: String,
    /// 出价资源类型。1表示人群定向，2表示资源位。
    #[serde(rename = "bidReferenceType")]
    pub bid_reference_type: i32,
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

impl RequestType for PddAdApiUnitBidQueryListRequest {
    type Response = PddAdApiUnitBidQueryListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.bid.query.list"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct AreaStruct {
    /// 地域Id列表。具体地域Id编码参见接口返回：pdd.ad.api.unit.bid.query.targeting.tag.list
    #[serde(default, rename = "areaIds")]
    pub area_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AdTargetingSet {
    /// 地域定向
    #[serde(default, rename = "areaStruct")]
    pub area_struct: Option<AreaStruct>,
}

#[derive(Debug, Deserialize)]
pub struct AdTargetingVO {
    /// 定向集合
    #[serde(default, rename = "adTargetingSet")]
    pub ad_targeting_set: Option<AdTargetingSet>,
    /// 定向Id
    #[serde(default, rename = "targetingId")]
    pub targeting_id: i64,
    /// 定向名称
    #[serde(default, rename = "targetingName")]
    pub targeting_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 广告单元Id
    #[serde(default, rename = "adId")]
    pub ad_id: i64,
    /// 定向信息
    #[serde(default, rename = "adTargetingVO")]
    pub ad_targeting_vo: Option<AdTargetingVO>,
    /// 每笔成交金额(average pay amount)，单位厘
    #[serde(default, rename = "avgPayAmount")]
    pub avg_pay_amount: f64,
    /// 出价Id
    #[serde(default, rename = "bidId")]
    pub bid_id: i64,
    /// 定向类型 或 资源位类型
    #[serde(default, rename = "bidReferenceId")]
    pub bid_reference_id: i64,
    /// 出价，万分比
    #[serde(default, rename = "bidValue")]
    pub bid_value: i64,
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
    /// 店铺收藏数
    #[serde(default, rename = "mallFavNum")]
    pub mall_fav_num: i64,
    /// 广告主Id
    #[serde(default, rename = "mallId")]
    pub mall_id: i64,
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 广告投入产出比
    #[serde(default)]
    pub roi: f64,
    /// 广告消耗，单位厘
    #[serde(default)]
    pub spend: i64,
    /// 人群定向二级Id。当出价资源为展示场景兴趣点时表示兴趣点Id，当出价资源为展示场景人群包时表示人群包Id，当出价资源为展示场景地域时表示地域Id。
    #[serde(default, rename = "subBidReferenceId")]
    pub sub_bid_reference_id: String,
    /// 人群定向二级名称。当出价资源为展示场景兴趣点时表示兴趣点名称，当出价资源为展示场景人群包时表示人群包名称，当出价资源为展示场景地域时表示地域名称。
    #[serde(default, rename = "subBidReferenceName")]
    pub sub_bid_reference_name: String,
    /// 转化成本
    #[serde(default, rename = "transactionCost")]
    pub transaction_cost: f64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitBidQueryListResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
