//! 创建全站推广广告

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitTrCreateRequest {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// 日限额（厘），范围100000~1000000000（表示100-1000000元）,选填
    #[serde(rename = "maxCost")]
    pub max_cost: Option<i64>,
    /// 成交出价（厘），4000~1000000(表示4-1000元)，选填
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
    /// 目标roi 范围1000~1000000 (万分位，表示0.1~100)，选填
    #[serde(rename = "targetRoi")]
    pub target_roi: Option<i64>,
}

impl RequestType for PddAdApiUnitTrCreateRequest {
    type Response = PddAdApiUnitTrCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.tr.create"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct MutexUnit {
    /// 广告id
    #[serde(default, rename = "adId")]
    pub ad_id: i64,
    /// 广告名称
    #[serde(default, rename = "adName")]
    pub ad_name: String,
    /// 广告场景
    #[serde(default, rename = "scenesType")]
    pub scenes_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 商品id
    #[serde(default, rename = "goodsId")]
    pub goods_id: i64,
    /// 全站推广互斥广告列表
    #[serde(default, rename = "mutexUnitList")]
    pub mutex_unit_list: Option<Vec<MutexUnit>>,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitTrCreateResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
