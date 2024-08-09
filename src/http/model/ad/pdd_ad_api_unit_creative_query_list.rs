//! 查询创意列表

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreativeQueryListRequest {
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

impl RequestType for PddAdApiUnitCreativeQueryListRequest {
    type Response = PddAdApiUnitCreativeQueryListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.creative.query.list"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
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
    /// 创意类型。2表示静态创意创意，3表示智能创意。
    #[serde(default, rename = "creativeType")]
    pub creative_type: i32,
    /// 广告点击率
    #[serde(default)]
    pub ctr: f64,
    /// 点击转化率
    #[serde(default)]
    pub cvr: f64,
    /// 启用状态。1表示启用，2表示暂停。
    #[serde(default, rename = "dataOperateStatus")]
    pub data_operate_status: i32,
    /// 广告转化支付金额，单位厘
    #[serde(default)]
    pub gmv: i64,
    /// 商品收藏数
    #[serde(default, rename = "goodsFavNum")]
    pub goods_fav_num: i64,
    /// 创意图片高
    #[serde(default, rename = "imageHeight")]
    pub image_height: i32,
    /// 创意图片链接
    #[serde(default, rename = "imageUrl")]
    pub image_url: String,
    /// 创意图片宽
    #[serde(default, rename = "imageWidth")]
    pub image_width: i32,
    /// 广告曝光量
    #[serde(default)]
    pub impression: i64,
    /// 店铺收藏数
    #[serde(default, rename = "mallFavNum")]
    pub mall_fav_num: i64,
    /// 广告转化支付订单量
    #[serde(default, rename = "orderNum")]
    pub order_num: i64,
    /// 广告投入产出比
    #[serde(default)]
    pub roi: f64,
    /// 广告消耗，单位厘
    #[serde(default)]
    pub spend: i64,
    /// 创意单元状态。1表示推广中，2表示手动暂停，3表示已删除，4表示待发布，5表示已驳回。
    #[serde(default)]
    pub status: i32,
    /// 状态描述，当状态是驳回时显示驳回原因
    #[serde(default, rename = "statusDesc")]
    pub status_desc: String,
    /// 创意标题
    #[serde(default)]
    pub title: String,
    /// 转化成本
    #[serde(default, rename = "transactionCost")]
    pub transaction_cost: f64,
    /// 创意单元Id
    #[serde(default, rename = "unitCreativeId")]
    pub unit_creative_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreativeQueryListResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
